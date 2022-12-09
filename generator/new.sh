# make it executable: chmod +x new.sh
# run it: ./new.sh
# get the folder name
echo "Enter the folder name: "
read folder_name

# create the folder
mkdir src/$folder_name

# create the mod.rs file
touch src/$folder_name/mod.rs

# create the readme file
touch src/$folder_name/README.md

# add the folder name to the main.rs file
echo "pub mod $folder_name;" >> src/main.rs

