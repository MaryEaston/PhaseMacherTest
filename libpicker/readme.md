# Picker

## 型

- 離散データ、離散重み
  - DataDigital = Hash<index: f32, value: f32>
- 連続データ
  - DataAnalog = fn(x: f32) -> y: f32
- データリスト
  - Datas = Hash<Key,impl Data>
- 点
  - Point = {x: f32, y: f32, kind: Kind}
- 点の種類
  - Kind = {Sharp, Series, None}

## 機能

- データ配列同士を重み配列に従って距離を算出
  - compare(data1: Data, data2: Data, weight: Data) -> length: f32
- 基準データリストのそれぞれに最も近いデータを探索データリストから取り出す
  - search_closest(reference: Datas, target: Datas, weight: Data) -> result: Datas
- データの作成
  - gen_series_data(points: Vec<Point>) -> DataS
  - 点の種類(左の点,自身,右の点)
    - (None, Sharp, None): todo
    - (None, Sharp, Sharp), (Sharp, Sharp, None): 左(右)の値→なし、右(左)の傾き→右(左)の点までの変化量
    - (None, Sharp, Series), (Series, Sharp, None): todo
    - (Sharp, Sharp, Sharp): todo
    - (Series, Sharp, Series): todo
    - (None, Series, None): todo
    - (None, Series, Sharp), (Sharp, Series, None): todo
    - (None, Series, Series), (Series, Series, None): todo
    - (Series, Series, Series): todo
    - (Sharp, Series, Sharp): todo
