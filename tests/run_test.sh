#!/bin/bash
set -e

if [ -z "$1" ]; then
    # 参数为空，获取所有 .sql 文件
    sql_files=(*.sql)
else
    # 参数不为空，使用传入的参数
    sql_files=("$@")
fi
for file in "${sql_files[@]}"; do
  echo "$file"
done
# exit 1

killall -q fatherduck || true
cargo build
echo "启动 cargo run"
../target/debug/fatherduck &
CARGO_PID=$!
echo "CARGO_PID: $CARGO_PID"

# 定义要检查的端口
PORT=55432

# 持续检查端口是否打开
while true; do
    # 检查端口是否打开
    if nc -z localhost $PORT; then
        echo "端口 $PORT 已打开。"
        break
    else
        echo "端口 $PORT 未打开，等待中..."
        sleep 1
    fi
done

# 定义目录名
output_dir="output"

# 检查目录是否存在
if [ -d "$output_dir" ]; then
    echo "$output_dir 目录存在，正在删除..."
    rm -rf "$output_dir"  # 使用 -r 递归删除目录及其内容
fi

# 新建目录
mkdir "$output_dir"
echo "$output_dir 目录已新建。"

prepare_file="prepare.sql"
# 遍历当前目录下所有 .sql 文件
for file in "${sql_files[@]}"; do
    if [[ "$file" == "$prepare_file" ]]; then
        continue
    fi
    echo -e "\n\n\n=============================执行SQL文件: $file 开始=============================\n"
    filename="${file%.sql}"
    # echo "找到 SQL 文件: $file, 文件名: $filename"
    mkdir $output_dir/$filename
    simple_file="$output_dir/$filename/${filename}_simple.sql"
    extented_file="$output_dir/$filename/${filename}_extended.sql"
    # cp $file $simple_file
    {
        echo "-- This is automatically generated, do not manually modify!!!"
        cat $prepare_file
        echo "-- after prepare"
        cat $file
    } > "$simple_file"
    {
        echo "-- This is automatically generated, do not manually modify!!!"
        cat $prepare_file
        echo "-- after prepare"
        sed 's/;/ \\bind \\g /g' "$file"
    } > "$extented_file"

    duckdb_log="$output_dir/$filename/${filename}_duckdb.log"
    simple_log="$output_dir/$filename/${filename}_simple.log"
    extented_log="$output_dir/$filename/${filename}_extended.log"
    duckdb_log_err="${duckdb_log}.err"
    simple_log_err="${simple_log}.err"
    extented_log_err="${extented_log}.err"


    ../libduckdb/duckdb -init /dev/null -csv -nullvalue '' -f "$simple_file" > $duckdb_log 2> "$duckdb_log_err"
    PGPASSWORD='fatherduck' psql -X -h 127.0.0.1 -p $PORT -U fatherduck -d database_name -q -f "$simple_file" --csv > $simple_log 2> "$simple_log_err"
    PGPASSWORD='fatherduck' psql -X -h 127.0.0.1 -p $PORT -U fatherduck -d database_name -q -f "$extented_file" --csv> $extented_log 2> "$extented_log_err"
    
    if [ -s "$duckdb_log_err" ]; then
        echo "出现错误 $duckdb_log_err:"
        cat "$duckdb_log_err"
        exit 1
    fi

    if [ -s "$simple_log_err" ]; then
        echo "出现错误 $simple_log_err:"
        cat "$simple_log_err"
        exit 1
    fi
    
    if [ -s "$extented_log_err" ]; then
        echo "出现错误 $extented_log_err:"
        cat "$extented_log_err"
        exit 1
    fi
    diff --color -u $duckdb_log $simple_log
    diff --color -u $simple_log $extented_log

    echo -e "=============================执行SQL文件: $file 结束============================="

done

echo "结束 cargo run"
kill $CARGO_PID

echo "全部测试成功!"
