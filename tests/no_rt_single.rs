#![ allow( unused_imports, dead_code ) ]
#![ feature( arbitrary_self_types, specialization, nll, never_type, unboxed_closures, trait_alias, box_syntax, box_patterns, todo_macro, try_trait, optin_builtin_traits ) ]

mod common;

use
{
	futures       :: { future::{ Future, FutureExt }, task::{ LocalSpawn, Spawn, SpawnExt, LocalSpawnExt }, executor::LocalPool } ,
	thespis       :: { * } ,
	log           :: { * } ,
	thespis_impl  :: { * } ,
	std           :: { pin::Pin         } ,
	common        :: actors::{ Sum, Add, Show   } ,
};



async fn sum_send( exec: &mut impl LocalSpawn ) -> u64
{
	let sum = Sum(5);

	// Create mailbox
	//
	let     mb  : Inbox<Sum> = Inbox::new(             );
	let mut addr             = Addr ::new( mb.sender() );

	// This is ugly right now. It will be more ergonomic in the future.
	//
	let move_mb = async move { mb.start_fut( sum ).await; };
	exec.spawn_local( move_mb ).expect( "Spawning mailbox failed" );


	addr.send( Add( 10 ) ).await.expect( "Send failed" );

	let res = addr.call( Show{} ).await.expect( "Call failed" );

	trace!( "res is: {}", res );

	res
}



async fn sum_call( exec: &mut impl LocalSpawn ) -> u64
{
	let sum = Sum(5);

	// Create mailbox
	//
	let     mb  : Inbox<Sum> = Inbox::new(             );
	let mut addr             = Addr ::new( mb.sender() );

	// This is ugly right now. It will be more ergonomic in the future.
	//
	let move_mb = async move { mb.start_fut( sum ).await; };
	exec.spawn_local( move_mb ).expect( "Spawning mailbox failed" );


	addr.call( Add( 10 ) ).await.expect( "Send failed" );

	let res = addr.call( Show{} ).await.expect( "Call failed" );

	trace!( "res is: {}", res );

	res
}



#[test]
//
fn test_basic_send()
{
	let mut pool = LocalPool::new();
	let mut exec = pool.spawner();

	let program = async move
	{
		// let _ = simple_logger::init();

		let result = sum_send( &mut exec ).await;

		trace!( "result is: {}", result );
		assert_eq!( 15, result );
	};

	pool.run_until( program );
}



#[test]
//
fn test_basic_call()
{
	let mut pool = LocalPool::new();
	let mut exec = pool.spawner();

	let program = async move
	{
		// let _ = simple_logger::init();

		let result = sum_call( &mut exec ).await;

		trace!( "result is: {}", result );
		assert_eq!( 15, result );
	};

	pool.run_until( program );
}
