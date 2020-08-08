extern crate lazy_static;

extern crate sgx_types;
extern crate sgx_urts;
use sgx_types::*;
use sgx_urts::SgxEnclave;

use std::env;
use std::io::BufRead;
use std::path::Path;
use std::ptr;
use lazy_static::lazy_static;

static ENCLAVE_FILE: &'static str = "enclave.signed.so";
lazy_static!{
  static ref SGX_ENCLAVE:SgxResult<SgxEnclave> =init_enclave();
}

extern "C" {
  fn do_query(
      eid: sgx_enclave_id_t,
      retval: *mut sgx_status_t,
      line: *const u8,
      len: usize,
      result_string: *const u8,
      result_max_len: usize,
      ) -> sgx_status_t;

  fn get_origin_by_id(
      eid: sgx_enclave_id_t,
      retval: *mut sgx_status_t,
      line: *const u8,
      len: usize,
      result_string: *const u8,
      result_max_len: usize,
      ) -> sgx_status_t;
}


#[no_mangle]
pub extern "C" fn init_enclave() -> SgxResult<SgxEnclave> {
  println!("init_enclave");

  let mut launch_token: sgx_launch_token_t = [0; 1024];
  let mut launch_token_updated: i32 = 0;
  // call sgx_create_enclave to initialize an enclave instance
  // Debug Support: set 2nd parameter to 1
  let debug = 1;
  let mut misc_attr = sgx_misc_attribute_t {
secs_attr: sgx_attributes_t { flags: 0, xfrm: 0 },
             misc_select: 0,
  };
  SgxEnclave::create(
      ENCLAVE_FILE,
      debug,
      &mut launch_token,
      &mut launch_token_updated,
      &mut misc_attr,
      )

}

#[no_mangle]
pub extern "C" fn rust_do_query( some_string:* const u8, some_len:usize,result_string_limit:usize, result_string: * mut u8,result_string_size: * mut usize ) -> Result<(),std::io::Error> {

  let v:&[u8]= unsafe { std::slice::from_raw_parts(some_string, some_len) };
  let line=String::from_utf8(v.to_vec()).unwrap();


  let enclave = match &*SGX_ENCLAVE {
    Ok(r) => {
      println!("[+] Init Enclave Successful {}!", r.geteid());
      r
    }
    Err(x) => {
      eprintln!("[-] Init Enclave Failed {}!", x.as_str());
      return Err(std::io::Error::new(std::io::ErrorKind::Other, "init enclave failed"));
    }
  };
  let enclave_id=enclave.geteid();

  let mut retval = sgx_status_t::SGX_SUCCESS;


  let result_max_len=1024;
  let mut result_vec:Vec<u8> = vec![0; result_max_len];
  let result_slice = &mut result_vec[..];

  let result = unsafe {
    do_query(
        enclave_id,
        &mut retval,
        line.as_ptr() as *const u8,
        line.len(),
        result_slice.as_mut_ptr() ,
        result_max_len
        )
  };

  match result {
    sgx_status_t::SGX_SUCCESS => {}
    _ => {
      eprintln!("[-] ECALL Enclave Failed {}!", result.as_str());
      return Err(std::io::Error::new(std::io::ErrorKind::Other, "ecall failed"));
    }
  }
  match retval{
    sgx_status_t::SGX_SUCCESS => {}
    e => {
      eprintln!("[-] ECALL Enclave Failed {}!", retval.as_str());
      return Err(std::io::Error::new(std::io::ErrorKind::Other, e.to_string()));
    }
  }

  let mut result_vec:Vec<u8> = result_slice.to_vec();
  result_vec.retain(|x| *x != 0x00u8);
  if result_vec.len() == 0 {
    println!("emptyString");
  }
  else{
    let raw_result_str = String::from_utf8(result_vec).unwrap();
      let l=raw_result_str.len();
      if l>result_string_limit{
        panic!("{} > {}",l,result_string_limit);
      }
    unsafe{
      *result_string_size=l;
      ptr::copy_nonoverlapping(raw_result_str.as_ptr(), result_string, raw_result_str.len());
    }

    // println!("{}",raw_result_str);
  }


  Ok(())

}

#[no_mangle]
pub extern "C" fn rust_search_title( some_string:* const u8, some_len:usize, result_string_limit:usize,result_string: * mut u8,result_string_size:* mut usize ) -> Result<(),std::io::Error> {

  let v:&[u8]= unsafe { std::slice::from_raw_parts(some_string, some_len) };
  let line=String::from_utf8(v.to_vec()).unwrap();

  let enclave = match &*SGX_ENCLAVE {
    Ok(r) => {
      println!("[+] Init Enclave Successful {}!", r.geteid());
      r
    }
    Err(x) => {
      println!("[-] Init Enclave Failed {}!", x.as_str());
      return Err(std::io::Error::new(std::io::ErrorKind::Other, "init enclave failed"));
    }
  };
  let enclave_id=enclave.geteid();

  let mut retval = sgx_status_t::SGX_SUCCESS;

  let result_max_len=1024;
  let mut result_vec:Vec<u8> = vec![0; result_max_len];
  let result_slice = &mut result_vec[..];


  let result = unsafe {
    get_origin_by_id (
        enclave_id,
        &mut retval,
        line.as_ptr() as *const u8,
        line.len(),
        result_slice.as_mut_ptr() ,
        result_max_len
        )
  };

  match result {
    sgx_status_t::SGX_SUCCESS => {}
    _ => {
      eprintln!("[-] ECALL Enclave Failed {}!", result.as_str());
      return Err(std::io::Error::new(std::io::ErrorKind::Other, "ecall failed"));
    }
  }
  match retval{
    sgx_status_t::SGX_SUCCESS => {}
    e => {
      eprintln!("[-] ECALL Enclave Failed {}!", retval.as_str());
      return Err(std::io::Error::new(std::io::ErrorKind::Other, e.to_string()));
    }
  }

  let mut result_vec:Vec<u8> = result_slice.to_vec();
  result_vec.retain(|x| *x != 0x00u8);
  if result_vec.len() == 0 {
    println!("emptyString");
  }
  else{
    let raw_result_str = String::from_utf8(result_vec).unwrap();
      let l=raw_result_str.len();
      if l>result_string_limit{
        panic!("{} > {}",l,result_string_limit);
      }
    unsafe{
      *result_string_size=l;
      ptr::copy_nonoverlapping(raw_result_str.as_ptr(), result_string, raw_result_str.len());
    }
    // println!("{}",raw_result_str);
  }

  Ok(())
}
