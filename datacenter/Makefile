main:
	cp *so /chaincode/output
	cp business/*.a /chaincode/output
	cp -r idx /chaincode/output

.PHONY: clean
clean:
	$(MAKE) -C sgx clean
	rm -rf enclave.signed.so libapp.a main go-sgx libapp.so go
