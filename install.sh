GIT_BRANCH=$(git symbolic-ref --short HEAD)

if [[ "$GIT_BRANCH" == "master" ]]; then
   PROGRAM=lazytf
else
   PROGRAM=lazytf-$GIT_BRANCH
fi

go build -o $PROGRAM
sudo cp ./$PROGRAM /usr/local/bin/
rm ./$PROGRAM

