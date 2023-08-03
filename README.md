# [Rust Atomics and Locks](https://marabos.nl/atomics/)

## GitHub

- [rust-atmics-and-locks](https://github.com/m-ou-se/rust-atomics-and-locks)

## Run

```bash
cargo run --example ${file_name}
```

or

```bash
cargo test ch4_spin_lock
```

## Reference

- [Rustの `Arc` を読む(4): アトミック変数とメモリ順序](https://qiita.com/qnighy/items/b3b728adf5e4a3f1a841)
- [データ競合と happens-before 関係](https://uchan.hateblo.jp/entry/2020/06/19/185152)
- コンピュータアーキテクチャの話
  - [メモリのアクセス時間とローカルメモリ](https://news.mynavi.jp/techplus/article/architecture-135/)
  - [ローカルメモリの2つの方法](https://news.mynavi.jp/techplus/article/architecture-136/)
  - [キャッシュの仕組み](https://news.mynavi.jp/techplus/article/architecture-137/)
  - [キャッシュアクセスの2つの考え方](https://news.mynavi.jp/techplus/article/architecture-138/)
  - [ダイレクトマップキャッシュとその注意点](https://news.mynavi.jp/techplus/article/architecture-139/)
  - [キャッシュの処理フロー](https://news.mynavi.jp/techplus/article/architecture-140/)
  - [空きキャッシュラインの確保](https://news.mynavi.jp/techplus/article/architecture-141/)
  - [ライトスルーとライトバック](https://news.mynavi.jp/techplus/article/architecture-142/)
  - [多階層のキャッシュ化](https://news.mynavi.jp/techplus/article/architecture-143/)
  - [メモリの管理機構](https://news.mynavi.jp/techplus/article/architecture-144/)
  - [フラグメンテーションとその解決法](https://news.mynavi.jp/techplus/article/architecture-145/)
  - [TLBの構造とTLBミスへの対応](https://news.mynavi.jp/techplus/article/architecture-146/)
  - [仮想アドレスキャッシュと物理アドレスキャッシュ](https://news.mynavi.jp/techplus/article/architecture-147/)
- ロードマップでわかる！当世プロセッサー事情
    - [CPU高速化の常套手段 パイプライン処理の基本 【その1】](https://ascii.jp/elem/000/000/552/552029/)
    - [CPU高速化の常套手段 パイプライン処理の基本 【その2】](https://ascii.jp/elem/000/000/553/553627/)
    - [スーパースカラーによる高速化とx86の問題点とは](https://ascii.jp/elem/000/000/555/555471/)
    - [命令の実行順を変えて高速化するアウトオブオーダー](https://ascii.jp/elem/000/000/557/557227/)
    - [x86を高速化する切り札技術「命令変換」の仕組み](https://ascii.jp/elem/000/000/558/558788/)
    - [Core iシリーズにも使われる「SMT」の利点と欠点](https://ascii.jp/elem/000/000/560/560386/)
    - [CPU性能向上のトレンド　マルチコアの理論と限界](https://ascii.jp/elem/000/000/561/561198/)
    - [CPUとメモリーの速度差を埋めるキャッシュの基礎知識](https://ascii.jp/elem/000/000/563/563800/)
    - [トランジスター数と性能を秤にかけるキャッシュ](https://ascii.jp/elem/000/000/566/566519/)
    - [仮想メモリーを支えるもうひとつのキャッシュ TLB](https://ascii.jp/elem/000/000/567/567889/)
    - [キャッシュの実装方式から見える AMDとインテルの置かれた状況](https://ascii.jp/elem/000/000/569/569422/)
    - [マルチコアCPUのキャッシュで問題となるコヒーレンシと解決策](https://ascii.jp/elem/000/000/571/571155/)

## 半順序と全順序

以下の性質を考える。ここで P は集合であり、「`≤`」を P 上で定義された二項関係(二変数関係)とする。

1. 反射律：P の任意の元(集合を構成する要素) `a` に対し、`a ≤ a` が成り立つ。
2. 反対称律：`a ≤ b` かつ `b ≤ a` ならば `a = b`
3. 推移律：`a ≤ b` かつ `b ≤ c` ならば `a ≤ c`
4. 完全律（比較可能）：`a ≤ b`または `b ≤ a` の何れかが必ず成り立つ

半順序は、反射律と推移律を満たし、且つ反対称律を満たすことをいう。
全順序は、半順序を満たし、且つ完全律(全順序律)を満たすことをいう。

## 推移閉包

推移閉包とは、集合Pにおける二項関係Rに対し、Rを含むP上の最小の推移関係を意味する。

## 推移関係

集合Pの二項関係Rが推移的であるとは、Xの任意の元a、b、cについて、aとbにRが成り立ち、bとcにRが成り立つとき、aとcにもRが成り立つことをいう。
