package business

import "C"
import (
	"crypto/rand"
	"encoding/json"
	"github.com/datacenter/model"
	"github.com/hyperledger/fabric-chaincode-go/shim"
	"github.com/hyperledger/fabric-protos-go/peer"
	"math/big"
	"unsafe"
)

//#cgo LDFLAGS: -L${SRCDIR} -L /opt/sgxsdk/lib64 -ltantivy -l sgx_urts -ldl
//#include <stdint.h>
//extern void rust_do_query( char* some_string, size_t some_len,size_t result_string_limit,char * result_string,size_t * result_string_size);
//extern void rust_search_title( char * some_string, size_t some_len,size_t result_string_limit,char * result_string, size_t * result_string_size);
//extern unsigned long long init_enclave();
import "C"

func LaunchSearchTask(stub shim.ChaincodeStubInterface, args []string) peer.Response {
	if len(args) != 1 {
		return shim.Error("invalid args")
	}
	err := stub.SetEvent(LauchSearchTaskEvent, []byte(args[0]))
	if err != nil {
		return shim.Error(err.Error())
	}
	return shim.Success(nil)
}

func QueryIDs(stub shim.ChaincodeStubInterface, args []string) peer.Response {
	if len(args) != 1 {
		return shim.Error("invalid args")
	}
	keyWord := C.CString(args[0])
	c := (*C.char)(C.malloc(ResultStringLimit))
	d := (C.ulong)(0)
	C.rust_do_query(keyWord, C.ulong(len(args[0])), ResultStringLimit, c, &d)
	str := C.GoBytes(unsafe.Pointer(c), (C.int)(d))
	return shim.Success([]byte(str))
}

func QueryData(stub shim.ChaincodeStubInterface, args []string) peer.Response {
	if len(args) != 1 {
		return shim.Error("invalid args")
	}
	cid := C.CString(args[0])
	c := (*C.char)(C.malloc(ResultStringLimit))
	d := (C.ulong)(0)
	C.rust_search_title(cid, C.ulong(len(args[0])), ResultStringLimit, c, &d)
	str := C.GoStringN(c, (C.int)(d))
	return shim.Success([]byte(str))
}

func mockSGX(keyWord string) (string, []string) {
	n, _ := rand.Int(rand.Reader, big.NewInt(128))
	if n.Int64()%2 == 0 {
		return "user1", []string{"a", "b", "c"}
	}
	return "user2", []string{"e", "d", "f"}
}

func mockSGX2(ids []string) string {
	return "abc"
}

func LogQuery(stub shim.ChaincodeStubInterface, args []string) peer.Response {
	if len(args) != 3 {
		return shim.Error("invalid args")
	}
	log := &model.QueryLog{
		Owner:     args[0],
		Payload:   args[1],
		TimeStamp: args[2],
	}
	logBytes, err := json.Marshal(log)
	if err != nil {
		return shim.Error(err.Error())
	}
	err = stub.PutState(string(logBytes), nil)
	if err != nil {
		return shim.Error(err.Error())
	}
	return shim.Success(nil)
}
