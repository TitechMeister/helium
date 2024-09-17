# Log Analysis Modules

# Project layout

## `analysis_log.py`

ログ解析

## `overlay.py`

ログを動画と合成

## `read_txt.py`

テキスト形式のログから各センサーの値と取得時刻を復元し、pandas.DataFrameに変換

## `cobs.py`

COBS(Consistent Overhead Byte Stuffing)エンコーダ

## `sensor.py`

各種センサのデータを表す構造体の定義

## `read_bin.py`

バイナリ形式のログから各センサーの値を復元

# Usage

## Preprocess

`helium`を実行した際にできる`log`ディレクトリを、この`python/`ディレクトリに丸ごと移動する。
こうすることにより、`analysis_log.py`や`overlay.py`を使うことができるようになる。

## `overlay.py`

以下の部分(11~14行目)を変更する
```python
    target='log/video/target.mp4'
    date='0608'
    JST = timezone(timedelta(hours=+9))
    start=datetime(2024,6,8,5,20,12,400000,tzinfo=JST)-timedelta(seconds=17.75)
```

- target
    合成する動画
- date
    フライトの日付
- start
    動画の開始時刻。回転数で判断するとよい。

ファイルを実行すると、output.mp4が出力されます。

## `analysis_log.py`

working progress