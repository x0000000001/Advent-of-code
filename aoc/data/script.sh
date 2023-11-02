for file in $(find . -name 'test_input.txt')
do
  mv $file $(echo "$file" | sed -r 's|test_input.txt|test_input0.txt|g')
done
