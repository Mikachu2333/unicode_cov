# Unicode Converter

本软件将帮助你转换 Unicode 编码（双向）。
This software would help in cov Unicode and Characters.

## 使用方法 Usage

### `unicode <Unicode chars> ...`

```shell
> unicode 0074 U+0065 u+0073 \U0074 \u793A [4F8B]


<t>     U+0074
<e>     U+0065
<s>     U+0073
<t>     U+0074
<示>    U+793A
<例>    U+4F8B
```

### `unicode <Unicode codes> ...`

```shell
> unicode 测试Test


测      试      T       e       s       t
<U+6D4B><U+8BD5><U+0054><U+0065><U+0073><U+0074>
```

### `unicode <Unicode codes> <Unicode chars> ... (Mixed)`

```shell
> unicode U+6D4B \u8BD5 Test


<测>    U+6D4B
<试>    U+8BD5
T       e       s       t
<U+0054><U+0065><U+0073><U+0074>
```
