package business

import (
  "reflect"
  "strconv"
	"encoding/json"
  "unsafe"
	"github.com/datacenter/model"
	"github.com/hyperledger/fabric-chaincode-go/shim"
	"github.com/hyperledger/fabric-protos-go/peer"
)

//#cgo LDFLAGS: -L${SRCDIR} -L /opt/sgxsdk/lib64 -lwrapper -l sgx_urts -ldl
//#include <stdint.h>
//extern int32_t rust_sgx_mpc(char* c, size_t l);
//extern unsigned long long init_enclave();
import "C"

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
  var mpcArgs model.MpcArgs
  mpcArgs.A = append(mpcArgs.A, args[0])
  mpcArgs.A = append(mpcArgs.A, args[1])
  argsBytes,err := json.Marshal(mpcArgs)
  if err != nil {
    return shim.Error("marshal failed")
  }
  argsString := string(argsBytes)
  p := (*reflect.StringHeader)(unsafe.Pointer(&argsString))
  res, _ := C.rust_sgx_mpc((*C.char)(unsafe.Pointer(p.Data)), C.ulong(len(argsString)))
	return shim.Success([]byte(strconv.FormatInt(int64(res),10)))
}
