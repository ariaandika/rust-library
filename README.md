# Creating Rust Library

Common pattern when creating rust library.

## Declaring Type

Declaring type may implement various traits.

Common type may implement:

- Debug, required
- From, TryFrom, when appropriate
- Clone, Display, Default, FromStr, when appropriate
- PartialEq, Eq, PartialOrd, Ord, Hash, for quality of life

Displayable type may implement:

- Display, when appropriate

Bytes kind of type may implement:

- Read

Buffer type may implement:

- Write

Wrapper type may implement:

- Deref, required
- DerefMut, when appropriate
- AsMut, AsRef, Borrow, ToOwned, for quality of life

Collection type may implement:

- Iterator, required
- FromIterator, Extend, Index, when appropriate
- IntoIterator, ExactSizeIterator, FusedIterator, for quality of life

Error type must implement:

- Error, Debug, Display

Future type may implement:

- Future, required
- IntoFuture, when appropriate

Other traits:

- Drop
- Copy, Send, Sync, Unpin

## Declaring Trait

Declaring trait may be implemented to various foreign type.

Common trait may be implemented to:

- unit, bool, integer, float, char, str, slice
- array, tuple
- Result, Option
- PhantomData, Infallible
- Box, Vec, VecDeque, String, Cow, HashMap, HashSet, BTreeMap
- NonZero, Atomic
- Cell, RefCell, Rc, Arc, Mutex, RwLock

Reference only trait may have blanket implementation for:

- &T
- &mut T, when appropriate

Owned trait may follow the From/Into pattern:

- FromTrait
- IntoTrait

