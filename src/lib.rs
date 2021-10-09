#![no_std]

#[macro_export]
macro_rules! req_evennumdig
{
	($e:expr) => {
		let _: $crate::internal::Even<[(); $e % 2]>;
	};
}
#[macro_export]
macro_rules! hex
{
	(@string $arg:expr) => {{
		const DAT: &[u8] = $arg.as_bytes();
		const RAWLEN: usize = $arg.len() - $crate::internal::count_skipped(&DAT);
		$crate::req_evennumdig!(RAWLEN);
		$crate::internal::convert::<{RAWLEN / 2}, {$arg.len()}>(&DAT)
	}};
	($($tt:tt)*) => {
		hex!(@string stringify!($($tt)*))
	};
}

pub mod internal
{
	pub type Even<T> = <<T as HexStrLen>::Marker as HexDigLenIsEven>::Check;
	pub enum IsEven {}
	pub enum IsOdd {}
	pub trait HexStrLen
	{
		type Marker;
	}
	impl HexStrLen for [(); 0]
	{
		type Marker = IsEven;
	}
	impl HexStrLen for [(); 1]
	{
		type Marker = IsOdd;
	}
	pub trait HexDigLenIsEven
	{
		type Check;
	}
	impl HexDigLenIsEven for IsEven
	{
		type Check = ();
	}
	pub const fn cntskip(data: &[u8]) -> usize
	{
		let mut charcnt: usize = 0;
		let mut charidx: usize = 0;
		while charidx < data.len()
		{
			if valdelim(data[charidx])
			{
				charcnt += 1;
			}
			charidx += 1;
		}
		charcnt
	}
	pub const fn valdelim(c: u8) -> bool
	{
		matches!(c, b' ' | b'"' | b'_' | b'|' | b'-' | b'\n')
	}
	#[allow(clippy::unnecessary_operation)]
	pub const fn toord(input: u8) -> u8
	{
		match input
		{
			b'-'..=b'9' => input - b'0',
			b'A'..=b'F' => input - b'A' + 10,
			b'a'..=b'f' => input - b'a' + 10,
			_ => {
				#[allow(unconditional_panic)]
				["[ERR] INVALID HEXADECIMAL DIGIT"][({ true } as usize)];
				loop {}
			}
		}
	}
	pub const fn conv<const RESSIZE: usize, const STRSIZE: usize>(input: &[u8]) -> [u8; RESSIZE]
	{
		let mut dat = [0_u8; RESSIZE];
		let mut datidx: usize = 0;
		let mut charidx: usize = 0;
		while datidx < STRSIZE && charidx + 1 < STRSIZE
		{
			if !valdelim(input[charidx])
			{
				let mut nextidx = charidx + 1;
				while nextidx < STRSIZE && valdelim(input[nextidx])
				{
					nextidx += 1;
				}
				dat[datidx] = toord(input[charidx]) * 16 + toord(input[nextidx]);
				charidx = nextidx + 1;
				datidx += 1;
			}
			else
			{
				charidx += 1;
			}
		}
		dat
	}
}
