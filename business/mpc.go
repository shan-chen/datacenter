package business

import (
	"encoding/json"
	"github.com/datacenter/model"
	"github.com/hyperledger/fabric-chaincode-go/shim"
	"github.com/hyperledger/fabric-protos-go/peer"
)

func UploadMpcData(stub shim.ChaincodeStubInterface, args []string) peer.Response {
	if len(args) != 4 {
		return shim.Error("invalid args")
	}
	data := &model.MpcTask{
		Nonce:   args[0],
		Owner:   args[1],
		Data:    args[2],
		Sponsor: args[3],
	}
	dataBytes, err := json.Marshal(data)
	if err != nil {
		return shim.Error("marshal failed")
	}
	err = stub.SetEvent("mpc", dataBytes)
	if err != nil {
		return shim.Error(err.Error())
	}
	return shim.Success(nil)
}

func ExecuteMpcTask(stub shim.ChaincodeStubInterface, args []string) peer.Response {
	if len(args) != TotalPeerNumber {
		return shim.Error("invalid args")
	}
	return shim.Success(nil)
}
