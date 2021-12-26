# 開発者用メモ(作者専用)


## subtree 用メモ


```
git remote add pac https://github.com/ryuz/pudding-pac.git

git subtree split --prefix pac --rejoin -b subtree/pac/master
git subtree merge --prefix pac subtree/pac/master

git push pac subtree/pac/master:master

git pull pac master:subtree/pac/master
```


## その他

```
git subtree add --prefix pac subtree/pac/master
```
