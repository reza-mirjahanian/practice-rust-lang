
**Debug**

-   Allows printing instances of your type using  `` `{:?}` ``
-   Can be derived for most types

**Clone**

-   Allows making copies of your type
-   Can be derived for most types

**Default**

-   Allows creating instances of your type with default values
-   Can be derived for most types

**PartialEq**

-   Allows comparing instances of your type for equality
-   Must be manually implemented

**Send**

-   Indicates your type is safe to send between threads
-   Automatically implemented for most types

**Sync**

-   Indicates your type is safe to share between threads via references
-   Automatically implemented for most types

**Additional Traits to Implement**

**Serialize** and **Deserialize**

-   From the  `` `serde` ``  crate, allow your type to be serialized and deserialized
-   Can be derived for most types

To implement these traits:

-   Add  `` `serde` ``  as a dependency
-   Derive  `` `Serialize` ``  and  `` `Deserialize` ``  for your types
-   Use  `` `serde_json` ``  to serialize/deserialize to/from JSON

To gate these traits behind a feature:

-   Make  `` `serde` ``  an optional dependency
-   Create a  `` `serde` ``  feature
-   Gate  `` `Serialize` ``  and  `` `Deserialize` ``  behind the  `` `serde` ``  feature
-   Users only pull in  `` `serde` ``  if they need serialization/deserialization