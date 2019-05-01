#[ derive( Debug, Clone, PartialEq ) ]
//
pub enum ConnectionError
{
	DeserializationFailure  ,
	LostRelayBeforeCall     ,
	LostRelayBeforeSend     ,
	LostRelayBeforeResponse ,
}
