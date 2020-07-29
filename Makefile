main:
	$(MAKE) -C sgx
	cp *so /chaincode/output
	cp *.a /chaincode/output
	cp -r sgx /chaincode/output

.PHONY: clean
clean:
	$(MAKE) -C sgx clean
	rm -rf enclave.signed.so libapp.a main go-sgx libapp.so go
