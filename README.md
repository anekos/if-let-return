# if-let-return

Simple macros for `if let ...`.


# Usage

## `if_let_some!`

```
pub fn read(&self, wrapped_data: Option<&str>) -> Vec<u8> {
  let data = if let Some(data) = wrapped_data {
    data
  } else {
    return vec![];
  }

  some_function(data);

  ...
}
```

↓

```
pub fn read(&self, wrapped_data: Option<&str>) -> Vec<u8> {
  if_let_some!(data = wrapped_data, vec![]);

  some_function(data);

  ...
}
```

## `if_let_ok!`

```
pub fn read(&self, wrapped_data: Result<&str, Error>) -> Vec<u8> {
  let data = match wrapped_data {
    Ok(data) => data,
    Err(err) => return err.to_vec(),
  };

  some_function(data);
  ...
}
```

↓

```
pub fn read(&self, wrapped_data: Result<&str, Error>) -> Vec<u8> {
  if_let_ok!(data = wrapped_data, |err| err.to_vec());

  some_function(data);
  ...
}
```
