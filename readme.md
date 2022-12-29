# Bisaya lang
isa ka Programming language nga ang gigamit kay bisaya

## Mga Example
### example code
```
ipasulod {
    random
}

deklara msg = "Wla kay "
deklara aha_diri = ["uyab", "kwarta"]

deklara resulta = msg + random.pilian(aha_diri)

kon resulta == "Wla kay uyab":
    ipakita("Gaba ray Ako") 
kon lain resulta == "Wla kay kwarta":
    ipakita("kaparehas rata")
lain:
    ipakita("Na, murag guba man guro ang Program") 
```
### functions
```
pamaaging puydi_ba (pangalan) {
    isulat("dli mag kamo, {}".format(pangalan))
}
```
## Errors
```
deklara numero = 1
deklara pangalan = "Name"
ipakita(numero + pangalang)
```
> linya 3, ika 15 na karakter:
> 
> Pisting Yawa, ayaw ug pag binugo, 
> ikaw daw add ug number sa letters.


## Credits
[The Expression Parser is Based on this](https://github.com/davidcallanan/py-myopl-code)