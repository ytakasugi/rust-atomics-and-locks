#/bin/bash
# ファイルリストを取得
FILE_LIST=`ls -1 ./examples`

for f in ${FILE_LIST}
do
  # ファイルサイズを取得
  SIZE=`ls -lh ./examples/${f} | awk '{print $5}'`
  # ファイルサイズが14byteでなければ、実行する
  if [ 14 -ne ${SIZE} ]; then
    TARGET_NAME=`basename ${f} .rs`
    cargo run --example ${TARGET_NAME}
  fi
done
