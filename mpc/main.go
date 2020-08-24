package main

import (
	"fmt"
	"github.com/hyperledger/fabric-chaincode-go/shim"
	"github.com/hyperledger/fabric-protos-go/peer"
	"github.com/mpc/business"
)

type MPC struct {
}

func (m *MPC) Init(stub shim.ChaincodeStubInterface) peer.Response {
	return shim.Success(nil)
}

func (m *MPC) Invoke(stub shim.ChaincodeStubInterface) peer.Response {
	fn, args := stub.GetFunctionAndParameters()

	switch fn {
	case "uploadMpcData":
		return business.UploadMpcData(stub, args)
	case "executeMpcTask":
		return business.ExecuteMpcTask(stub, args)
	default:
		return shim.Error("invalid method")
	}
}

func main() {
	if err := shim.Start(new(MPC)); err != nil {
		fmt.Printf("Error starting MetaData chaincode: %s", err)
	}
}
