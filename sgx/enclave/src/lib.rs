// Licensed to the Apache Software Foundation (ASF) under one
// or more contributor license agreements.  See the NOTICE file
// distributed with this work for additional information
// regarding copyright ownership.  The ASF licenses this file
// to you under the Apache License, Version 2.0 (the
// "License"); you may not use this file except in compliance
// with the License.  You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing,
// software distributed under the License is distributed on an
// "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.  See the License for the
// specific language governing permissions and limitations
// under the License..

#![crate_name = "helloworldsampleenclave"]
#![crate_type = "staticlib"]
#![cfg_attr(not(target_env = "sgx"), no_std)]
#![cfg_attr(target_env = "sgx", feature(rustc_private))]

extern crate sgx_trts;
extern crate sgx_types;
#[cfg(not(target_env = "sgx"))]
#[macro_use]
extern crate sgx_tstd as std;

extern crate lazy_static;
extern crate tantivy;
extern crate serde;

use sgx_trts::enclave;
use sgx_types::metadata::*;
use sgx_types::*;
//use sgx_trts::{is_x86_feature_detected, is_cpu_feature_supported};
use std::backtrace::{self, PrintFormat};
use std::io::{self, Write};
use std::slice;
use std::string::String;
use std::vec::Vec;
//use std::fs;
use std::path::{Path, PathBuf};
use std::sync::SgxRwLock as RwLock;
use std::sync::Arc;
use std::ptr;

use tantivy::tokenizer::TokenizerManager;
use tantivy::collector::{Count, TopDocs};
use tantivy::query::QueryParser;
use tantivy::query::TermQuery;
use tantivy::schema::*;
use tantivy::{doc, Index, ReloadPolicy,IndexWriter,IndexReader,Searcher,LeasedItem};
use tantivy::merge_policy::NoMergePolicy;

use serde::{Serialize, Deserialize};

use lazy_static::lazy_static;

#[derive(Serialize, Deserialize, Debug)]
struct Point {
A: std::vec::Vec<String>,
}

lazy_static!{
    static ref schema:Schema={
    let mut schema_builder = Schema::builder();

    schema_builder.add_text_field("id", STRING|STORED);
    schema_builder.add_text_field("text", TEXT|STORED);

    schema_builder.build()
    };
   
    static ref index:Index = {
    std::untrusted::fs::create_dir_all("idx").map_err(|e| {
          panic!(e);
    });
    let index_path = match tantivy::directory::MmapDirectory::open(Path::new("idx")){
      Ok(index_path)=>index_path,
        Err(e) =>{
          panic!(e);
        }
    };

     let x=match Index::open_or_create(index_path, schema.clone()){
        Ok(index1)=>index1,
        Err(e) =>{
          panic!(e);
          }
      };
     x
    };

    static ref index_writer:Arc<RwLock<IndexWriter>> = Arc::new(RwLock::new(
          match index.writer(2_000_000_000){
          Ok(index_writer1)=>{
            index_writer1.set_merge_policy(std::boxed::Box::new(NoMergePolicy));
            index_writer1
          },
          Err(e)=>{ panic!(e); }
          }
          ));

    static ref query_parser:QueryParser = {
      let text_field = index.schema().get_field("text").expect("no all field?!");
      QueryParser::new(
          index.schema(),
          vec![text_field],
          TokenizerManager::default())
    };

    static ref reader:IndexReader  = {
        match index
        .reader_builder()
        .reload_policy(ReloadPolicy::Manual)
        .try_into() {
          Ok(reader1)=>reader1,
          Err(e)=>{panic!(e);}
        }
    };
}



#[no_mangle]
pub extern "C" fn build_index(some_string:* const u8,some_len:usize) -> sgx_status_t {
    let v:&[u8]= unsafe { slice::from_raw_parts(some_string, some_len) };
    let line=String::from_utf8(v.to_vec()).unwrap();


    // Let's index our documents!
    // We first need a handle on the title and the body field.

    // ### Adding documents
    //
    // We can create a document manually, by setting the fields
    // one by one in a Document object.

    let doc = match schema.parse_document(&line){
      Ok(doc)=>doc,
        _=>{
        return sgx_status_t::SGX_ERROR_UNEXPECTED;
        }
    };

    let index_writer_clone_1 = index_writer.clone();
    index_writer_clone_1.read().unwrap().add_document(doc);


    // This is an example, so we will only index 3 documents
    // here. You can check out tantivy's tutorial to index
    // the English wikipedia. Tantivy's indexing is rather fast.
    // Indexing 5 million articles of the English wikipedia takes
    // around 3 minutes on my computer!

    // ### Committing
    //
    // At this point our documents are not searchable.
    //
    //
    // We need to call `.commit()` explicitly to force the
    // `index_writer` to finish processing the documents in the queue,
    // flush the current index to the disk, and advertise
    // the existence of new documents.
    //
    // This call is blocking.

    sgx_status_t::SGX_SUCCESS
}

#[no_mangle]
pub extern "C" fn commit() -> sgx_status_t {
    let index_writer_clone_2 = index_writer.clone();
    index_writer_clone_2.write().unwrap().commit().map_err(
        |e|{
        eprintln!("{}", e);
        return sgx_status_t::SGX_ERROR_UNEXPECTED;
      });

    // index_writer.read().unwrap().wait_merging_threads().map_err(|e|{
        // eprintln!("{}", e);
        // return sgx_status_t::SGX_ERROR_UNEXPECTED;
      // });

    sgx_status_t::SGX_SUCCESS
}

#[no_mangle]
pub extern "C" fn do_query(some_string:* const u8,some_len:usize,
                           result_string:* mut u8,result_max_len:usize,
                          ) -> sgx_status_t {
    let v:&[u8]= unsafe { slice::from_raw_parts(some_string, some_len) };
    let line=String::from_utf8(v.to_vec()).unwrap();

    reader.reload().unwrap();
    let searcher = reader.searcher();

     let mut point = Point { A: vec![]};

    let query = match query_parser.parse_query(&line){
      Ok(query)=>query,
        Err(e)=>{panic!(e);}
    };

    //suppose 1024 is large enough
    let top_docs= match searcher.search(&query, &TopDocs::with_limit(1024)){
      Ok(top_docs)=>top_docs,
      Err(e)=>{panic!(e);}
    };

    let id= schema.get_field("id").unwrap();

    for (_score, doc_address) in top_docs {
      let retrieved_doc = searcher.doc(doc_address).map_err(|e|{
          eprintln!("{}", e);
          return sgx_status_t::SGX_ERROR_UNEXPECTED;
          }).unwrap();

      let g= retrieved_doc.get_first(id).unwrap().text().unwrap();
      point.A.push(String::from(g));
    }

    let x= serde_json::to_string(&point).unwrap();

      if x.len() < result_max_len {
        unsafe {
          ptr::copy_nonoverlapping(x.as_ptr(), result_string, x.len());
        }
        return sgx_status_t::SGX_SUCCESS
      }
      else{
        println!("Result len = {} > buf size = {}", x.len(), result_max_len);
        return sgx_status_t::SGX_ERROR_WASM_BUFFER_TOO_SHORT;
      }

    sgx_status_t::SGX_SUCCESS
}

#[no_mangle]
pub extern "C" fn get_origin_by_id(some_string:* const u8,some_len:usize,
                                  result_string:* mut u8,result_max_len:usize,
                                  ) -> sgx_status_t {
    let v:&[u8]= unsafe { slice::from_raw_parts(some_string, some_len) };
    let line=String::from_utf8(v.to_vec()).unwrap();

    let id= schema.get_field("id").unwrap();
    let text = schema.get_field("text").unwrap();

    let frankenstein_isbn = Term::from_field_text(id,&line);
    let frankenstein_doc_misspelled = extract_doc_given_id(&reader, &frankenstein_isbn).map_err(|e|{
                  panic!(e);
                          }).unwrap();

    if frankenstein_doc_misspelled.is_none(){
      println!("isnone");
        return sgx_status_t::SGX_ERROR_WASM_BUFFER_TOO_SHORT;
    }

      let y=frankenstein_doc_misspelled.unwrap();
      let x=y.get_first(text).unwrap().text().unwrap();

      if x.len() < result_max_len {
        unsafe {
          ptr::copy_nonoverlapping(x.as_ptr(), result_string, x.len());
        }
        return sgx_status_t::SGX_SUCCESS
      }
      else{
        println!("Result len = {} > buf size = {}", x.len(), result_max_len);
        return sgx_status_t::SGX_ERROR_WASM_BUFFER_TOO_SHORT;
      }

}

fn extract_doc_given_id(
        indexreader: &IndexReader,
            isbn_term: &Term,
                ) -> tantivy::Result<Option<Document>> {
  let searcher = indexreader.searcher();

  // This is the simplest query you can think of.
  // It matches all of the documents containing a specific term.
  //
  // The second argument is here to tell we don't care about decoding positions,
  // or term frequencies.
  let term_query = TermQuery::new(isbn_term.clone(), IndexRecordOption::Basic);
  let top_docs = searcher.search(&term_query, &TopDocs::with_limit(1))?;

  if let Some((_score, doc_address)) = top_docs.first() {
      let doc = searcher.doc(*doc_address)?;
      Ok(Some(doc))
    } else {
        // no doc matching this ID.
        Ok(None)
      }
}

