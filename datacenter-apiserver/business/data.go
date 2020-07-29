package business

import (
	"github.com/datacenter-apiserver/common"
	"github.com/datacenter-apiserver/model"
	"github.com/gin-gonic/gin"
	"github.com/hyperledger/fabric-sdk-go/pkg/client/channel"
	log "github.com/sirupsen/logrus"
	"net/http"
	"sync"
	"time"
)

type DataController struct {
	dataChan   map[string]chan model.MetaData
	metaData   map[string][]model.MetaData
	searchDone map[string]chan struct{}
	waitNumber map[string]int
	mu         sync.Mutex
}

func (dc *DataController) SearchByKeyWords(ctx *gin.Context) {
	keyWord := ctx.Query("keyword")
	dc.mu.Lock()
	if _, ok := dc.dataChan[keyWord]; ok {
		dc.waitNumber[keyWord] += 1
		dc.mu.Unlock()
		t := time.NewTimer(common.TimeOut * time.Second)
		select {
		case <-t.C:
			log.Warn("previous search timeout")
			ctx.JSON(common.TimeOutErr, "timeout")
			return
		case <-dc.searchDone[keyWord]:
			ctx.JSON(http.StatusOK, dc.metaData[keyWord])
			return
		}
	} else {
		waitChan := make(chan model.MetaData, common.TotalPeerNumber)
		dc.dataChan[keyWord] = waitChan
		dc.mu.Unlock()
		_, err := ExecuteChainCode(common.LaunchSearchTask, [][]byte{[]byte(keyWord)})
		if err != nil {
			ctx.JSON(common.ExecuteChainCodeErr, err.Error())
			return
		}
		res := dc.waitMetaData(keyWord)
		dc.metaData[keyWord] = res
		ch := make(chan struct{}, 1000)
		dc.searchDone[keyWord] = ch
		dc.mu.Lock()
		for i := 0; i < dc.waitNumber[keyWord]; i++ {
			ch <- struct{}{}
		}
		delete(dc.dataChan, keyWord)
		dc.mu.Unlock()
		ctx.JSON(http.StatusOK, res)
	}
}

func (dc *DataController) waitMetaData(keyWord string) []model.MetaData {
	count := 0
	res := make([]model.MetaData, 0)
	t := time.NewTimer(common.TimeOut * time.Second)
	for {
		select {
		case data := <-dc.dataChan[keyWord]:
			count += 1
			res = append(res, data)
			if count == common.TotalPeerNumber {
				return res
			}
		case <-t.C:
			return res
		}
	}
}

func (dc *DataController) SearchCallBack(ctx *gin.Context) {
	var metaData model.MetaData
	if err := ctx.ShouldBindJSON(&metaData); err != nil {
		log.WithError(err).Error("bind json failed")
		return
	}
	dc.dataChan[metaData.KeyWord] <- metaData
}

func (dc *DataController) GetDataDetail(ctx *gin.Context) {
	title := ctx.Query("title")
	owner := ctx.Query("owner")
	if title == "" || owner == "" {
		log.WithField("title", title).WithField("owner", owner).Error("invalid query args")
		ctx.JSON(common.InvalidArgErr, "invalid query args")
		return
	}
	resp, err := QueryChainCode("QueryData", [][]byte{[]byte(title)}, owner)
	if err != nil {
		log.WithError(err).Error("query failed")
		ctx.JSON(common.QueryChainCodeErr, "query failed")
		return
	}
	if len(resp.Responses) <= 0 || resp.Responses[0] == nil || resp.Responses[0].Response == nil {
		log.Error("query result is nil")
		ctx.JSON(common.QueryChainCodeErr, "query result is nil")
		return
	}
	data := resp.Responses[0].Response.Payload
	ctx.JSON(http.StatusOK, data)
}

func QueryChainCode(method string, args [][]byte, targetPeer string) (channel.Response, error) {
	req := channel.Request{
		ChaincodeID: common.ChainCodeID,
		Fcn:         method,
		Args:        args,
	}
	return channelClient.Query(req, channel.WithTargetEndpoints(targetPeer))
}

func ExecuteChainCode(method string, args [][]byte) (channel.Response, error) {
	req := channel.Request{
		ChaincodeID: common.ChainCodeID,
		Fcn:         method,
		Args:        args,
	}
	return channelClient.Execute(req)
}
