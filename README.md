# simple-redis

## 一个简单的redis

### get/set command
```zsh
127.0.0.1:6379> set key value
OK
127.0.0.1:6379> get key
"value"
```

### hget/hset/hmget/hgetall command
```zsh
127.0.0.1:6379> hset myhash key value
OK
127.0.0.1:6379> hset myhash key1 value1
OK
127.0.0.1:6379> hset myhash key2 value2
OK
127.0.0.1:6379> hget myhash key
"value"
127.0.0.1:6379> hmget myhash key key1 key2 key3
1) "value"
2) "value1"
3) "value2"
4) (nil)
127.0.0.1:6379> hgetall myhash
1) "key"
2) "value"
3) "key2"
4) "value2"
5) "key1"
6) "value1"
```

### sadd/sismember command
```zsh
127.0.0.1:6379> sadd myhash 1 2 3 4 "5" "six"
(integer) 6
127.0.0.1:6379> sismember myhash six
(integer) 1
127.0.0.1:6379> sismember myhash 0
(integer) 0
```
