# funcao recursiva com caso base e caso recursivo.
def regressiva(i):
    print(str(i) + "...")
    if i <= 1:
        return
    else:
        regressiva(i - 1)

regressiva(5)

# Exemplificação da call stack
def sauda(nome):
	print("Olá, " + nome + "!")
	sauda2(nome)
	print("preparando para dizer tchau...")
	tchau()
	
def sauda2(nome):
	print("Como vai " + nome + "?") 

def tchau():
	print("ok, tchau!")
