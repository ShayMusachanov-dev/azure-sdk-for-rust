# Microsoft Azure SDK for Rust

[![Build Status](https://travis-ci.org/MindFlavor/AzureSDKForRust.svg?branch=master)](https://travis-ci.org/MindFlavor/AzureSDKForRust) [![Coverage Status](https://coveralls.io/repos/MindFlavor/AzureSDKForRust/badge.svg?branch=master&service=github)](https://coveralls.io/github/MindFlavor/AzureSDKForRust?branch=master) [![experimental](http://badges.github.io/stability-badges/dist/experimental.svg)](http://github.com/badges/stability-badges)

## Introduction
Microsoft Azure expose its technologies via REST API. These APIs are easily consumable from any language (good) but are weakly typed. With this library and its related (as soon as I figure out how to make it) crate you can exploit the power of Microsoft Azure from Rust in a idiomatic way.    

> **NOTE:** This repository is under heavy ongoing development and
is likely to break over time. I currently do not have any releases
yet.

## Disclaimer
Although I am a Microsoft employee, this is not a Microsoft endorsed project. It's simply a pet project of mine: I love Rust (who doesn't? :smirk:) and Microsoft Azure technologies so I thought to close the gap between them. It's also a good project for learning Rust.

## Example
You can find examples in the test section (not yet existent as far as Azure is concerned) and in the main.rs file. Here is a sample however:

```rust
use azure::storage::client;
use azure::storage::container::PublicAccess;

fn main() {
	let azure_storage_account = "azure_storage_account".to_owned();
	let azure_storage_key= "azure_storage_key".to_owned();

	// create the client struct. The third argument, if false, forces to use
	// http instead of https. It's useful if you have trouble compiling
	// hyper with openSSL activated.
    let client = client::new(azure_storage_account, azure_storage_key, false);

	// This call will create a new Azure Container called "wow"
	// with public blob access (see https://msdn.microsoft.com/en-us/library/azure/dd179468.aspx).
    client.create_container("wow", PublicAccess::Blob).unwrap();

	// This call will list your containers.
    let mut ret = client.list_containers().unwrap();
    println!("{:?}", ret);

	// This code will look for the "todelete" container and
	// remove from Azure.
    let mut to_delete = ret.iter_mut().find(|x| x.name == "todelete").unwrap();
    to_delete.delete(&client).unwrap();
    println!("{:?} deleted!", to_delete);
}
```

## State of the art
Right now very few methods have been implemented but the key framework is in place (authentication, enumerations, parsing and so on). If you want to contribute please do!
Also note that the project is in early stages so the APIs are bound to change at any moment. I will strive to keep things steady but since I'm new to Rust I'm sure I'll have to correct some serious mistake before too long :smile:.

## Contributing
If you want to contribute please do! No formality required! :wink:

### Implemented methods

#### Storage Container

|Method | URL |
| ----  | --- |
|Create container|[https://msdn.microsoft.com/en-us/library/azure/dd179468.aspx]()|
|List containers|[https://msdn.microsoft.com/en-us/library/azure/dd179352.aspx]()|
|Delete container|[https://msdn.microsoft.com/en-us/library/azure/dd179408.aspx]()|

#### Storage blobs

|Method | URL |
| ----  | --- |
|List blobs|[https://msdn.microsoft.com/en-us/library/azure/dd135734.aspx]()|

## License
This project is published under [The MIT License (MIT)](LICENSE).
