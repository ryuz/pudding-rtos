# 開発者用メモ(作者専用)


## subtree 用メモ


```
git remote add pac    https://github.com/ryuz/pudding-pac.git
git remote add kernel https://github.com/ryuz/pudding-kernel.git

git subtree split --prefix pac    --rejoin -b subtree/pac/master
git subtree split --prefix kernel --rejoin -b subtree/kernel/master

git subtree merge --prefix pac    subtree/pac/master
git subtree merge --prefix kernel subtree/kernel/master

git push pac    subtree/pac/master:master
git push kernel subtree/kernel/master:master

git pull pac    master:subtree/pac/master
git pull kernel master:subtree/kernel/master
```


## その他

```
git subtree add --prefix pac subtree/pac/master
```
