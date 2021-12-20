i=1
while [ $i -lt 81 ]; do curl www.setgame.com/sites/all/modules/setgame_set/assets/images/new/$i.png -o $i.png && i=$[$i+1];done
