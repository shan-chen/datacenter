module github.com/datacenter-client

go 1.14

require (
	github.com/hyperledger/fabric-sdk-go v1.0.0-beta2
	github.com/sirupsen/logrus v1.6.0 // indirect
)

replace github.com/hyperledger/fabric-sdk-go => ../hyperledger/fabric-sdk-go
