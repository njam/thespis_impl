#![ feature( await_macro, async_await, futures_api, arbitrary_self_types, specialization, nll, never_type, unboxed_closures, trait_alias ) ]

#![ allow( dead_code, unused_imports )]


use
{
	futures       :: { future::{ Future, FutureExt }, task::{ LocalSpawn, Spawn, SpawnExt }, executor::LocalPool } ,
	thespis       :: { * } ,
	log           :: { * } ,
	thespis_impl  :: { single_thread::* } ,
	std           :: { pin::Pin } ,
};


#[ derive( Actor ) ]
//
struct Sum( u64 );

#[ derive( Debug ) ] struct Add( u64 );
#[ derive( Debug ) ] struct Show;

impl Message for Add  { type Result = ();  }
impl Message for Show { type Result = u64; }



impl Handler< Add > for Sum
{
	fn handle( &mut self, msg: Add ) -> Response<Add> { async move
	{
		trace!( "called sum with: {:?}", msg );

		self.0 += msg.0;

	}.boxed() }
}



impl Handler< Show > for Sum
{
	fn handle( &mut self, _msg: Show ) -> Response<Show> { async move
	{
		trace!( "called sum with: Show" );

		self.0

	}.boxed() }
}



impl Drop for Sum
{
	fn drop( &mut self )
	{
		trace!( "dropping Sum")
	}
}



async fn sum( exec: &mut impl Spawn ) -> u64
{
	let sum = Sum(5);

	let mut mb  : ProcLocalMb<Sum>   = sum.start( exec );
	let mut addr: ProcLocalAddr<Sum> = mb.addr();

	await!( addr.call( Add( 10 ) ) );

	let res = await!( addr.call( Show{} ) );

	trace!( "res is: {}", res );
	return res;
	// return await!( addr.call( Show{} ) );
}



#[test]
//
fn test_basic_send_call()
{
	let mut pool = LocalPool::new();
	let mut exec = pool.spawner();

	let program = async move
	{
		simple_logger::init().unwrap();

		let result = await!( sum( &mut exec ) );

		trace!( "result is: {}", result );
		assert_eq!( 15, result );
	};

	pool.run_until( program );
}