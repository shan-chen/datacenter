package main

import (
	"fmt"
	"github.com/datacenter/business"
	"github.com/hyperledger/fabric-chaincode-go/shim"
	"github.com/hyperledger/fabric-protos-go/peer"
)

type Datacenter struct {
}

func (dc *Datacenter) Init(stub shim.ChaincodeStubInterface) peer.Response {
	return shim.Success(nil)
}

func (dc *Datacenter) Invoke(stub shim.ChaincodeStubInterface) peer.Response {
	fn, args := stub.GetFunctionAndParameters()

	switch fn {
	case "launchSearchTask":
		return business.LaunchSearchTask(stub, args)
	case "queryIDs":
		return business.QueryIDs(stub, args)
	case "queryData":
		return business.QueryData(stub, args)
	case "logQuery":
		return business.LogQuery(stub, args)
	default:
		return shim.Error("invalid method")
	}
}

func main() {
	if err := shim.Start(new(Datacenter)); err != nil {
		fmt.Printf("Error starting MetaData chaincode: %s", err)
	}
}
