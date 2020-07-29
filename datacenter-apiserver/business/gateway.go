package business

import (
	"github.com/datacenter-apiserver/common"
	"github.com/datacenter-apiserver/model"
	"github.com/gin-gonic/gin"
	"github.com/hyperledger/fabric-sdk-go/pkg/client/channel"
	"github.com/hyperledger/fabric-sdk-go/pkg/core/config"
	"github.com/hyperledger/fabric-sdk-go/pkg/fabsdk"
	log "github.com/sirupsen/logrus"
)

var dc *DataController
var channelClient *channel.Client

func InitSDK() {
	dc = new(DataController)
	dc.dataChan = make(map[string]chan model.MetaData)
	dc.metaData = make(map[string][]model.MetaData)
	dc.searchDone = make(map[string]chan struct{})
	dc.waitNumber = make(map[string]int)
	sdk, err := fabsdk.New(config.FromFile(common.ConfigPath))
	if err != nil {
		log.WithError(err).Error("cannot load config file")
		return
	}
	ccp := sdk.ChannelContext(common.ChannelID, fabsdk.WithOrg(common.OrgName), fabsdk.WithUser(common.UserName))
	channelClient, err = channel.New(ccp)
	if err != nil {
		log.WithError(err).Error("cannot get channel client")
		return
	}
}

func SearchByKeyWord(ctx *gin.Context) {
	dc.SearchByKeyWords(ctx)
}

func SearchCallBack(ctx *gin.Context) {
	dc.SearchCallBack(ctx)
}

func GetDataDetail(ctx *gin.Context) {
	dc.GetDataDetail(ctx)
}
