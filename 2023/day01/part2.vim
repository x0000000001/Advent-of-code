:%s/one/one1one/g
:%s/two/two2two/g
:%s/three/three3three/g
:%s/four/four4four/g
:%s/five/five5five/g
:%s/six/six6six/g
:%s/seven/seven7seven/g
:%s/eight/eight8eight/g
:%s/nine/nine9nine/g

qai_<ESC>A_<ESC>q "Enregistre une macro a qui ajoute '_' au début et à la fin d'une ligne
:2,$ normal @a "Applique la macro a à toutes les lignes
/\d<ENTER> "Active la recherche sur les chiffres
qb0n"xyl$N"yylA<CTRL-R>=@x*10+@y<ENTER><ESC>q "Enregistre une macro b qui calcule premier chiffre * 10 + dernier chiffre d'une ligne
:2,$ normal @b "Applique la macro b à toutes les lignes
ggO0<ESC> "Ajoute une ligne avec 0 au début du fichier
j "Descend d'une ligne
qc$h"lywgg0"syw<SHIFT-S><CTRL-R>=@l+@s<ENTER><ESC>q "Ajoute le nombre de chaque ligne calculé avec la macro b à la première ligne 
:3,$ normal @c "Applique cette macro à toutes les lignes
