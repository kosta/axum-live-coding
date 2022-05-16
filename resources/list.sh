#! /bin/sh
while read -r name; do
    while read -r animal; do
        while read -r color; do
            while read -r fruit; do
                echo "$color $fruit $name $animal";
                sleep 0.1;
            done <resources/fruits.txt
        done <resources/colors.txt
    done<resources/animals.txt
done <resources/names.txt
