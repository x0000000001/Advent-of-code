sed 's/one/one1one/g; s/two/two2two/g; s/three/three3three/g; s/four/four4four/g; 
  s/five/five5five/g; s/six/six6six/g; s/seven/seven7seven/g; s/eight/eight8eight/g; 
  s/nine/nine9nine/g' $1 |
	sed -E 's/[a-z]*([0-9]).*([0-9])[a-z]*/\1\2/; s/^[a-z]*([0-9])[a-z]*$/\1\1/' |
	awk '{s+=$1} END {print s}'
