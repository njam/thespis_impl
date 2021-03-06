use
{
	async_executors   :: { * } ,
	actix             :: { Actor as AxActor, Message as AxMessage, Handler as AxHandler, Context as AxContext, Arbiter } ,
	criterion         :: { Criterion, Benchmark, criterion_group, criterion_main } ,
	futures           :: { future::{ TryFutureExt }, compat::Future01CompatExt, executor::{ block_on }, task::LocalSpawnExt } ,
	thespis           :: { * } ,
	thespis_impl      :: { * } ,
};



#[ derive( Actor ) ] struct Sum( u64 );

struct Add (u64);
struct Show     ;

impl Message for Add  { type Return = () ; }
impl Message for Show { type Return = u64; }


impl Handler< Add > for Sum
{
	fn handle( &mut self, msg: Add ) -> Return<()> { Box::pin( async move
	{

		self.0 += msg.0;

	})}
}


impl Handler< Show > for Sum
{
	fn handle( &mut self, _msg: Show ) -> Return<u64> { Box::pin( async move
	{

		self.0

	})}
}


struct Accu( u64 );

impl Accu
{
	#[ inline( never ) ]
	//
	async fn add( &mut self, v: Add )
	{
		self.0 += v.0;
	}

	#[ inline( always ) ]
	//
	async fn add_inline( &mut self, v: Add )
	{
		self.0 += v.0;
	}


	async fn show( &mut self ) -> u64
	{
		self.0
	}
}


fn send_localpool()
{
	let mut pool = LocalPool::default();
	let     exec = pool.clone();

	let bench = async move
	{
		let     sum              = Sum(5)                  ;
		let     mb  : Inbox<Sum> = Inbox::new()            ;
		let mut addr             = Addr::new( mb.sender() );

		// This is ugly right now. It will be more ergonomic in the future.
		//
		exec.spawn_local( mb.start_fut( sum ) ).expect( "Spawning failed" );

		for _i in 0..100usize
		{
			addr.send( Add( 10 ) ).await.expect( "Send failed" );
		}

		let res = addr.call( Show{} ).await.expect( "Call failed" );
		assert_eq!( 1005, res );
	};

	pool.spawn_local( bench ).expect( "Spawn benchmark" );

	pool.run();
}


fn call_localpool()
{
	let mut pool = LocalPool::default();
	let     exec = pool.clone();

	let bench = async move
	{
		let     sum              = Sum(5)                  ;
		let     mb  : Inbox<Sum> = Inbox::new()            ;
		let mut addr             = Addr::new( mb.sender() );

		// This is ugly right now. It will be more ergonomic in the future.
		//
		exec.spawn_local( mb.start_fut( sum ) ).expect( "Spawning failed" );

		for _i in 0..100usize
		{
			addr.call( Add( 10 ) ).await.expect( "Call failed" );
		}

		let res = addr.call( Show{} ).await.expect( "Call failed" );
		assert_eq!( 1005, res );
	};

	pool.spawn_local( bench ).expect( "Spawn benchmark" );

	pool.run();
}


fn send_tokio_ct()
{
	let mut pool = TokioCt::default();
	let     exec = pool.clone();

	let bench = async move
	{
		let     sum              = Sum(5)                  ;
		let     mb  : Inbox<Sum> = Inbox::new()            ;
		let mut addr             = Addr::new( mb.sender() );

		// This is ugly right now. It will be more ergonomic in the future.
		//
		exec.spawn_local( mb.start_fut( sum ) ).expect( "Spawning failed" );

		for _i in 0..100usize
		{
			addr.send( Add( 10 ) ).await.expect( "Send failed" );
		}

		let res = addr.call( Show{} ).await.expect( "Call failed" );
		assert_eq!( 1005, res );
	};

	pool.spawn_local( bench ).expect( "Spawn benchmark" );

	pool.run().expect( "run TokioCt" );
}


fn call_tokio_ct()
{
	let mut pool = TokioCt::default();
	let     exec = pool.clone();

	let bench = async move
	{
		let     sum              = Sum(5)                  ;
		let     mb  : Inbox<Sum> = Inbox::new()            ;
		let mut addr             = Addr::new( mb.sender() );

		// This is ugly right now. It will be more ergonomic in the future.
		//
		exec.spawn_local( mb.start_fut( sum ) ).expect( "Spawning failed" );

		for _i in 0..100usize
		{
			addr.call( Add( 10 ) ).await.expect( "Call failed" );
		}

		let res = addr.call( Show{} ).await.expect( "Call failed" );
		assert_eq!( 1005, res );
	};

	pool.spawn_local( bench ).expect( "Spawn benchmark" );

	pool.run().expect( "run TokioCt" );
}


fn actix_dosend()
{
	actix::System::run( ||
	{
		Arbiter::spawn( Box::pin( async
		{
			let sum  = AxSum(5)    ;
			let addr = sum.start() ;

			for _i in 0..100usize
			{
				addr.do_send( AxAdd( 10 ) );
			}

			let res = addr.send( AxShow{} ).compat().await.unwrap();

			assert_eq!( 1005, res );

			actix::System::current().stop();

			Ok(())

		}).compat());

	}).unwrap();
}


fn actix_send()
{
	actix::System::run( ||
	{
		Arbiter::spawn( Box::pin( async
		{
			let sum  = AxSum(5)    ;
			let addr = sum.start() ;

			for _i in 0..100usize
			{
				addr.send( AxAdd( 10 ) ).compat().await.unwrap();
			}

			let res = addr.send( AxShow{} ).compat().await.unwrap();

			assert_eq!( 1005, res );

			actix::System::current().stop();

			Ok(())

		}).compat());

	}).unwrap();
}


fn method()
{
	block_on( async
	{
		let mut sum = Accu(5);

		for _i in 0..100usize
		{
			sum.add( Add( 10 ) ).await;
		}

		let res = sum.show().await;
		assert_eq!( 1005, res );
	})
}


fn inline_method()
{
	block_on( async
	{
		let mut sum = Accu(5);

		for _i in 0..100usize
		{
			sum.add_inline( Add( 10 ) ).await;
		}

		let res = sum.show().await;
		assert_eq!( 1005, res );
	})
}


// --------------------------------------------------------------------

struct AxSum (u64);
struct AxAdd (u64);
struct AxShow     ;

impl AxMessage for AxAdd  { type Result  = ()              ; }
impl AxMessage for AxShow { type Result  = u64             ; }
impl AxActor   for AxSum  { type Context = AxContext<Self> ; }


impl AxHandler< AxAdd > for AxSum
{
	type Result  = ()
;
	fn handle( &mut self, msg: AxAdd, _ctx: &mut AxContext<Self> )
	{
		self.0 += msg.0;
	}
}


impl AxHandler< AxShow > for AxSum
{
	type Result  = u64
;
	fn handle( &mut self, _msg: AxShow, _ctx: &mut AxContext<Self> ) -> Self::Result
	{
		self.0
	}
}




fn bench_calls( c: &mut Criterion )
{
	c.bench
	(
		"Single Thread Delivery",

		Benchmark::new   ( "Send LocalPool x100"     , |b| b.iter( || send_localpool () ) )
			.with_function( "Call LocalPool x100"     , |b| b.iter( || call_localpool () ) )
			.with_function( "Send TokioCt x100"       , |b| b.iter( || send_tokio_ct  () ) )
			.with_function( "Call TokioCt x100"       , |b| b.iter( || call_tokio_ct  () ) )
			.with_function( "async method x100"       , |b| b.iter( || method         () ) )
			.with_function( "async inline method x100", |b| b.iter( || inline_method  () ) )
			.with_function( "actix do_send x100"      , |b| b.iter( || actix_dosend   () ) )
			.with_function( "actix send x100"         , |b| b.iter( || actix_send     () ) )
	);
}

criterion_group!( benches, bench_calls );
criterion_main! ( benches              );
