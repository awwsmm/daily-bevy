# daily-bevy

Learn [Bevy](https://bevyengine.org/) by exploring a small example (almost) every day.

[Bevy](https://github.com/bevyengine/bevy/) is a free, open-source, cross-platform (Windows, macOS, Linux, Web, iOS, Android) game engine written in [Rust](https://www.rust-lang.org/).

This README shows the fourteenth entry in this series. All other entries can be found at [daily-bevy/branches](https://github.com/awwsmm/daily-bevy/branches).

## Reflection

Today is the fourteenth day of Daily Bevy.

This kata uses [Bevy `v0.12.1`](https://github.com/bevyengine/bevy/tree/v0.12.1).

### Today's Kata

Today, we'll be taking a closer look at [`reflection`](https://github.com/bevyengine/bevy/blob/v0.12.1/examples/reflection/reflection.rs) in Bevy.

#### The Code

Here's the `main.rs` for this example, with comments and imports removed

```rust
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .register_type::<Foo>()
        .register_type::<Bar>()
        .add_systems(Startup, setup)
        .run();
}

#[derive(Reflect)]
#[reflect(from_reflect = false)]
pub struct Foo {
    a: usize,
    nested: Bar,
    #[reflect(ignore)]
    _ignored: NonReflectedValue,
}

#[derive(Reflect)]
pub struct Bar {
    b: usize,
}

#[derive(Default)]
pub struct NonReflectedValue {
    _a: usize,
}

fn setup(type_registry: Res<AppTypeRegistry>) {
    let mut value = Foo {
        a: 1,
        _ignored: NonReflectedValue { _a: 10 },
        nested: Bar { b: 8 },
    };

    *value.get_field_mut("a").unwrap() = 2usize;
    assert_eq!(value.a, 2);
    assert_eq!(*value.get_field::<usize>("a").unwrap(), 2);

    let field = value.field("a").unwrap();

    assert_eq!(*field.downcast_ref::<usize>().unwrap(), 2);

    let mut patch = DynamicStruct::default();
    patch.insert("a", 4usize);

    value.apply(&patch);
    assert_eq!(value.a, 4);

    let type_registry = type_registry.read();
    let serializer = ReflectSerializer::new(&value, &type_registry);
    let ron_string =
        ron::ser::to_string_pretty(&serializer, ron::ser::PrettyConfig::default()).unwrap();
    info!("{}\n", ron_string);

    let reflect_deserializer = UntypedReflectDeserializer::new(&type_registry);
    let mut deserializer = ron::de::Deserializer::from_str(&ron_string).unwrap();
    let reflect_value = reflect_deserializer.deserialize(&mut deserializer).unwrap();

    let _deserialized_struct = reflect_value.downcast_ref::<DynamicStruct>();

    assert!(reflect_value.reflect_partial_eq(&value).unwrap());

    value.apply(&*reflect_value);
}
```

And here's the `Cargo.toml` for this example

```toml
[dependencies]
bevy = "0.12.1"
ron = "0.8.1"
serde = "1.0.196"
```

This example uses the `serde` crate for general de/serialization, and the `ron` crate for de/serialization to and from [Rusty Object Notation (RON)](https://github.com/ron-rs/ron). 

#### Discussion

Yesterday, we saw how reflection can be used in Bevy to save and load `Scene`s to and from files. Today, we'll dig further into reflection and see how we can use the popular `serde` crate alongside Bevy.

```rust
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .register_type::<Foo>()
        .register_type::<Bar>()
        .add_systems(Startup, setup)
        .run();
}
```

Like yesterday, we

- create an `App`
- add the `DefaultPlugins`, which means a `Window` will be rendered
- register some types
- add a `Startup` system, and
- `run` the app

Then, again like yesterday, we define some types: `Foo`, `Bar`, and `NonReflectedValue`. Let's start with `Foo`.

---

```rust
/// Deriving `Reflect` implements the relevant reflection traits. In this case, it implements the
/// `Reflect` trait and the `Struct` trait `derive(Reflect)` assumes that all fields also implement
/// Reflect.
///
/// All fields in a reflected item will need to be `Reflect` as well. You can opt a field out of
/// reflection by using the `#[reflect(ignore)]` attribute.
/// If you choose to ignore a field, you need to let the automatically-derived `FromReflect` implementation
/// how to handle the field.
/// To do this, you can either define a `#[reflect(default = "...")]` attribute on the ignored field, or
/// opt-out of `FromReflect`'s auto-derive using the `#[reflect(from_reflect = false)]` attribute.
#[derive(Reflect)]
#[reflect(from_reflect = false)]
pub struct Foo {
    a: usize,
    nested: Bar,
    #[reflect(ignore)]
    _ignored: NonReflectedValue,
}
```

We saw some of this in the last kata. Most of the above is pretty self-explanatory, but how does `reflect(default = "...")` work? Luckily, there's an example in the Bevy repo

```rust
#[test]
fn from_reflect_should_use_default_field_attributes() {
    #[derive(Reflect, Eq, PartialEq, Debug)]
    struct MyStruct {
        // Use `Default::default()`
        // Note that this isn't an ignored field
        #[reflect(default)]
        foo: String,

        // Use `get_bar_default()`
        #[reflect(ignore)]
        #[reflect(default = "get_bar_default")]
        bar: NotReflect,

        // Ensure attributes can be combined
        #[reflect(ignore, default = "get_bar_default")]
        baz: NotReflect,
    }

    #[derive(Eq, PartialEq, Debug)]
    struct NotReflect(usize);

    fn get_bar_default() -> NotReflect {
        NotReflect(123)
    }

    let expected = MyStruct {
        foo: String::default(),
        bar: NotReflect(123),
        baz: NotReflect(123),
    };

    let dyn_struct = DynamicStruct::default();
    let my_struct = <MyStruct as FromReflect>::from_reflect(&dyn_struct);

    assert_eq!(Some(expected), my_struct);
}
```

The above test shows how we might use a `reflect(default = ...)` attribute to deserialize a default value.

Note that we refer to the `get_bar_default` function by its _name_, in a string: `"get_bar_default"`.

---

`Bar` is shorter, and has a constraint on it

```rust
/// This `Bar` type is used in the `nested` field on the `Test` type. We must derive `Reflect` here
/// too (or ignore it)
#[derive(Reflect)]
pub struct Bar {
    b: usize,
}
```

We _must_ `#[derive(Reflect)]` for `Bar` because `Foo` is `#[derive(Reflect)]` and _contains_ a `Bar`. (Alternatively, as the comment notes, we could `ignore` the `Bar` field of `Foo`.)

---

Finally, we've got `NonReflectedValue`

```rust
#[derive(Default)]
pub struct NonReflectedValue {
    _a: usize,
}
```

This `struct` does not `#[derive(Reflect)]`, which is fine, because it is `ignore`d in `Foo`.

---

There's only one `system` in this example. Here's its signature

```rust
fn setup(type_registry: Res<AppTypeRegistry>) {
    // -- snip --
}
```

It only takes one argument, an immutable reference to the `AppTypeRegistry` `Res`ource.

Within `setup`, we start by defining an instance of `Foo` called `value`

```rust
let mut value = Foo {
    a: 1,
    _ignored: NonReflectedValue { _a: 10 },
    nested: Bar { b: 8 },
};
```

The next bit of code is where things start to get interesting

```rust
// You can set field values like this. The type must match exactly or this will fail.
*value.get_field_mut("a").unwrap() = 2usize;
assert_eq!(value.a, 2);
assert_eq!(*value.get_field::<usize>("a").unwrap(), 2);
```

"The type must match exactly or this will fail" -- what happens if we change `2usize` to just `2`?

```
thread 'Compute Task Pool (0)' panicked at src/main.rs:66:31:
called `Option::unwrap()` on a `None` value
```

That's weird... why are we getting a `None` value error?

The type of `*value.get_field_mut("a").unwrap()` is inferred by the compiler. If you inspect this expression in your IDE, you should see that it has type `i32`, which makes sense because `2` is of type `i32`, but the `a` field on `Foo` is of type `usize`... so how does Bevy resolve this?

Well, `get_field_mut` looks like this

```rust
fn get_field_mut<T: Reflect>(&mut self, name: &str) -> Option<&mut T> {
    self.field_mut(name)
        .and_then(|value| value.downcast_mut::<T>())
}
```

Note that it takes a type `T: Reflect`, and then does a downcast of the `value` if finds in the field with the specified `name`.

`value` is of type `&mut dyn Reflect` before it is downcast. It could be any type which implements `Reflect`. So to return a value of the specific type `T`, we need to explicitly convert the type. This is what `downcast_mut` does

```rust
/// Downcasts the value to type `T` by mutable reference.
///
/// If the underlying value is not of type `T`, returns `None`.
#[inline]
pub fn downcast_mut<T: Reflect>(&mut self) -> Option<&mut T> {
    self.as_any_mut().downcast_mut::<T>()
}
```

The nested `downcast_mut` comes directly from the Rust `stdlib` (namely `core`)

```rust
#[stable(feature = "rust1", since = "1.0.0")]
#[inline]
pub fn downcast_mut<T: Any>(&mut self) -> Option<&mut T> {
    if self.is::<T>() {
        // SAFETY: just checked whether we are pointing to the correct type, and we can rely on
        // that check for memory safety because we have implemented Any for all types; no other
        // impls can exist as they would conflict with our impl.
        unsafe { Some(self.downcast_mut_unchecked()) }
    } else {
        None
    }
}
```

This is where the `None` comes from -- if `self` is not of type `T`, this method returns `None`.

Even further up the call stack, `downcast_mut_unchecked` takes the mutable reference to `self`, turns it into a mutable pointer to a `dyn Any` object, and then downcasts that to a mutable pointer to a `T`

```rust
#[unstable(feature = "downcast_unchecked", issue = "90850")]
#[inline]
pub unsafe fn downcast_mut_unchecked<T: Any>(&mut self) -> &mut T {
    debug_assert!(self.is::<T>());
    // SAFETY: caller guarantees that T is the correct type
    unsafe { &mut *(self as *mut dyn Any as *mut T) }
}
```

So _most_ of this is Rust standard library stuff, Bevy just wraps it.

But don't overlook the main point of this code -- we accessed (and mutated) the field `a` on an instance of `Foo` using only its name contained in a string (`"a"`)! This is what reflection lets us do.

---

Reflection also lets us _get_ these values, as well as set them, of course

```rust
// You can also get the &dyn Reflect value of a field like this
let field = value.field("a").unwrap();

// you can downcast Reflect values like this:
assert_eq!(*field.downcast_ref::<usize>().unwrap(), 2);
```

Above, we just read the value of the `a` field from our `Foo` `value`.

---

The next bit seems innocuous until you look at it a bit harder

```rust
// DynamicStruct also implements the `Struct` and `Reflect` traits.
let mut patch = DynamicStruct::default();
patch.insert("a", 4usize);
```

`DynamicStruct` is not a type we've defined. This is a Bevy-defined `struct`

```rust
/// A struct type which allows fields to be added at runtime.
#[derive(Default)]
pub struct DynamicStruct {
    represented_type: Option<&'static TypeInfo>,
    fields: Vec<Box<dyn Reflect>>,
    field_names: Vec<Cow<'static, str>>,
    field_indices: HashMap<Cow<'static, str>, usize>,
}
```

...but it's a Bevy-defined struct which lets us dynamically add fields at runtime!

```rust
/// Inserts a field named `name` with the typed value `value` into the struct.
///
/// If the field already exists, it is overwritten.
pub fn insert<T: Reflect>(&mut self, name: &str, value: T) {
    if let Some(index) = self.field_indices.get(name) {
        self.fields[*index] = Box::new(value);
    } else {
        self.insert_boxed(name, Box::new(value));
    }
}
```

...and we can even do things like...

```rust
// You can "apply" Reflect implementations on top of other Reflect implementations.
// This will only set fields with the same name, and it will fail if the types don't match.
// You can use this to "patch" your types with new values.
value.apply(&patch);
assert_eq!(value.a, 4);
```

`apply` is quite a complex method, which acts differently based on whether the value being applied is a struct, enum, tuple, etc.

```rust
    /// Applies a reflected value to this value.
    ///
    /// If a type implements a subtrait of `Reflect`, then the semantics of this
    /// method are as follows:
    /// - If `T` is a [`Struct`], then the value of each named field of `value` is
    ///   applied to the corresponding named field of `self`. Fields which are
    ///   not present in both structs are ignored.
    /// - If `T` is a [`TupleStruct`] or [`Tuple`], then the value of each
    ///   numbered field is applied to the corresponding numbered field of
    ///   `self.` Fields which are not present in both values are ignored.
    /// - If `T` is an [`Enum`], then the variant of `self` is `updated` to match
    ///   the variant of `value`. The corresponding fields of that variant are
    ///   applied from `value` onto `self`. Fields which are not present in both
    ///   values are ignored.
    /// - If `T` is a [`List`] or [`Array`], then each element of `value` is applied
    ///   to the corresponding element of `self`. Up to `self.len()` items are applied,
    ///   and excess elements in `value` are appended to `self`.
    /// - If `T` is a [`Map`], then for each key in `value`, the associated
    ///   value is applied to the value associated with the same key in `self`.
    ///   Keys which are not present in `self` are inserted.
    /// - If `T` is none of these, then `value` is downcast to `T`, cloned, and
    ///   assigned to `self`.
    ///
    /// Note that `Reflect` must be implemented manually for [`List`]s and
    /// [`Map`]s in order to achieve the correct semantics, as derived
    /// implementations will have the semantics for [`Struct`], [`TupleStruct`], [`Enum`]
    /// or none of the above depending on the kind of type. For lists and maps, use the
    /// [`list_apply`] and [`map_apply`] helper functions when implementing this method.
    ///
    /// [`list_apply`]: crate::list_apply
    /// [`map_apply`]: crate::map_apply
    ///
    /// # Panics
    ///
    /// Derived implementations of this method will panic:
    /// - If the type of `value` is not of the same kind as `T` (e.g. if `T` is
    ///   a `List`, while `value` is a `Struct`).
    /// - If `T` is any complex type and the corresponding fields or elements of
    ///   `self` and `value` are not of the same type.
    /// - If `T` is a value type and `self` cannot be downcast to `T`
    fn apply(&mut self, value: &dyn Reflect);
```

---

What else can we do with reflection in Bevy? It can make de/serialization easier.

We don't need to derive `serde`'s `Serialize` trait -- Bevy does this for us with `Reflect`

```rust
let type_registry = type_registry.read();
// By default, all derived `Reflect` types can be Serialized using serde. No need to derive
// Serialize!
let serializer = ReflectSerializer::new(&value, &type_registry);
let ron_string =
    ron::ser::to_string_pretty(&serializer, ron::ser::PrettyConfig::default()).unwrap();
info!("{}\n", ron_string);
```

Above, we use the `serde` and `ron` crates to serialize our `Foo` `value` to Rusty Object Notation (RON)

```ron
{
    "daily_bevy::Foo": (
        a: 4,
        nested: (
            b: 8,
        ),
    ),
}
```

One more thing here, though: why do we need to `.read()` the `type_registry`? Can't we use it directly? If we drop this line

```rust
let type_registry = type_registry.read();
```

we get the following compilation error

```
89 |     let serializer = ReflectSerializer::new(&value, &type_registry);
   |                      ----------------------         ^^^^^^^^^^^^^^ expected `&TypeRegistry`, found `&Res<'_, AppTypeRegistry>`
   |                      |
   |                      arguments to this function are incorrect
```

...so we need to unwrap the `&Res<'_, T>` type into a `&T` in order to pass it as an argument to `ReflectSerializer::new()`. That's what `.read()` does for us. 

```rust
/// Takes a read lock on the underlying [`TypeRegistry`].
pub fn read(&self) -> RwLockReadGuard<'_, TypeRegistry> {
    self.internal.read().unwrap_or_else(PoisonError::into_inner)
}
```

---

The rest of this example took me a few re-reads to understand. Here it is in full

```rust
// Dynamic properties can be deserialized
let reflect_deserializer = UntypedReflectDeserializer::new(&type_registry);
let mut deserializer = ron::de::Deserializer::from_str(&ron_string).unwrap();
let reflect_value = reflect_deserializer.deserialize(&mut deserializer).unwrap();

// Deserializing returns a Box<dyn Reflect> value. Generally, deserializing a value will return
// the "dynamic" variant of a type. For example, deserializing a struct will return the
// DynamicStruct type. "Value types" will be deserialized as themselves.
let _deserialized_struct = reflect_value.downcast_ref::<DynamicStruct>();

// Reflect has its own `partial_eq` implementation, named `reflect_partial_eq`. This behaves
// like normal `partial_eq`, but it treats "dynamic" and "non-dynamic" types the same. The
// `Foo` struct and deserialized `DynamicStruct` are considered equal for this reason:
assert!(reflect_value.reflect_partial_eq(&value).unwrap());

// By "patching" `Foo` with the deserialized DynamicStruct, we can "Deserialize" Foo.
// This means we can serialize and deserialize with a single `Reflect` derive!
value.apply(&*reflect_value);
```

First, we create an `UntypedReflectDeserializer` from the `type_registry`

```rust
/// A general purpose deserializer for reflected types.
///
/// This will return a [`Box<dyn Reflect>`] containing the deserialized data.
/// For non-value types, this `Box` will contain the dynamic equivalent. For example, a
/// deserialized struct will return a [`DynamicStruct`] and a `Vec` will return a
/// [`DynamicList`]. For value types, this `Box` will contain the actual value.
/// For example, an `f32` will contain the actual `f32` type.
///
/// This means that converting to any concrete instance will require the use of
/// [`FromReflect`], or downcasting for value types.
///
/// Because the type isn't known ahead of time, the serialized data must take the form of
/// a map containing the following entries (in order):
/// 1. `type`: The _full_ [type path]
/// 2. `value`: The serialized value of the reflected type
///
/// If the type is already known and the [`TypeInfo`] for it can be retrieved,
/// [`TypedReflectDeserializer`] may be used instead to avoid requiring these entries.
///
/// [`Box<dyn Reflect>`]: crate::Reflect
/// [`DynamicStruct`]: crate::DynamicStruct
/// [`DynamicList`]: crate::DynamicList
/// [`FromReflect`]: crate::FromReflect
/// [type path]: crate::TypePath::type_path
pub struct UntypedReflectDeserializer<'a> {
    registry: &'a TypeRegistry,
}
```

...so this lets us deserialize RON-formattted (and maybe otherly-formatted?) data to one of these `Dynamic` types. We then do this exact thing in the next line

```rust
let mut deserializer = ron::de::Deserializer::from_str(&ron_string).unwrap();
```

But the above is a `Deserializer`, and we use it again in the next line, where we call a `deserialize` method

```rust
let reflect_value = reflect_deserializer.deserialize(&mut deserializer).unwrap();
```

So my guess is that

- `reflect_deserializer` is a generic deserializer, which knows about the `type_registry`
- `deserializer` is a RON deserializer, which turns the RON string into some Rust data, and then
- `reflect_deserializer.deserialize(&mut deserializer)` does a "two step" deserialization, where the RON string is turned into generic Rust data, and then converted to a specific type using the information in the `type_registry`

...but that's just a guess. This is all a bit unclear to me.

Next, as `reflect_value` is of the type `Box<dyn Reflect>`, we want to convert that to something more concrete. We do that by explicitly downcasting to a `DynamicStruct`

```rust
let _deserialized_struct = reflect_value.downcast_ref::<DynamicStruct>();
```

...and as the comments in this section explain, this `DynamicStruct` can be compared to the equivalent "compile-time struct" using `reflect_partial_eq`, which compares the two structs field-by-field, ignoring the fact that one of them was defined at compile-time and one of them was dynamically defined at runtime

```rust
assert!(reflect_value.reflect_partial_eq(&value).unwrap());
```

This last bit, I initially didn't understand the significance of

```rust
// By "patching" `Foo` with the deserialized DynamicStruct, we can "Deserialize" Foo.
// This means we can serialize and deserialize with a single `Reflect` derive!
value.apply(&*reflect_value);
```

Yes, we can "patch" compile-time structs with runtime structs. That I understand. 

> we can "Deserialize" Foo

...we've already done this, right? That's what `reflect_value` is.

But `reflect_value` is of type `Box<dyn Reflect>`, and `value` is of type `Foo`. So I guess what this is saying is that, since we can `apply` `reflect_value` as a "patch" to `value`, we can deserialize RON data to a correctly-typed `Foo` instance.

Neat!

---

As the comment at the top of `main.rs` notes

> "Reflection is a core part of Bevy and enables a number of interesting features"

We've explored how we can de/serialize data in Bevy using `serde` and `ron`, and dynamically add and update fields to a struct at runtime.

Bevy's powerful reflection tools let us "bend the rules" of strictly-typed Rust, and surely this will come in handy as we write more Bevy code.

## Learn More

If you found this first kata interesting, head over to [daily-bevy/branches](https://github.com/awwsmm/daily-bevy/branches) to see the rest of them.

If you have questions, comments, or corrections, please head over to [daily-bevy/discussions](https://github.com/awwsmm/daily-bevy/discussions) to join the conversation.

If you like what you've read above, you can [follow me on Bluesky](https://bsky.app/profile/awwsmm.bsky.social) or [Mastodon](https://mas.to/@awwsmm).
