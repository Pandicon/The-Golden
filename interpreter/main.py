import sys
from copy import deepcopy
import re

class Lexer:
	def __init__(self, text, rules):
		self.text = text
		self.rules = rules
		self.line = 1
		self.column = 1
		self.pos = 0
		self.comment = False
	def next(self):
		t = self.text[self.pos:]
		if len(t) < 1:
			return None
		if t == '"':
			self.comment = not self.comment
		if self.comment:
			return None
		for r in self.rules:
			m = re.match(r, t)
			if m:
				s = m.group(0)
				self.pos += len(s)
				if '\n' in s:
					self.line += s.count('\n')
					self.column += len(s.rsplit('\n', 1)[-1])
				else:
					self.column += len(s)
				return (s, self.line, self.column)
		raise SyntaxError("Syntax error at %d:%d" % (self.line, self.column))

class Validator:
	def run(self, lex: Lexer):
		p = None
		t = lex.next()
		while t:
			p = t
			t = lex.next()
		if not re.match(":\r?\n?", p[0]):
			raise SyntaxError("Syntax error at %d:%d" % (lex.line, lex.column))

class Parser:
	def __init__(self):
		self.whiles = {}
		self.do_whiles = {}
		self.commands = []

	def run(self, lex: Lexer):
		t = lex.next()
		while t:
			print(t)
			if '"' in t[0] or ":" in t[0]:
				t = lex.next()
				continue
			self.commands.append(t[0])
			t = lex.next()
		print("Commands:")
		print(self.commands)

class AUI:
	def __init__(self):
		self.linea = [0.0]
		self.lineb = [0.0]
		self.ptra = 0
		self.ptrb = 0
		self.ptr_mem = 0
	def run(self, lex: Lexer):
		p = None
		t = lex.next()
		while t:
			if t[0] == "!":
				self.linea[self.ptra] += 1
			if t[0] == "^":
				self.ptr_mem = abs(self.ptr_mem - 1)
			p = t
			t = lex.next()
		if p[0] != ":" and p[0] != ":\n" and p[0] != ":\r\n":
			raise SyntaxError("Syntax error at %d:%d" % (lex.line, lex.column))

if __name__ == "__main__":
	args = sys.argv
	debug_heavy = False
	flags = []
	possible_flags = [
		"--debug",
		"-"
	]
	args_amount = len(args)
	for i in range(0, args_amount):
		arg = args[i]
		if arg == "-" and not "-" in flags and i < args_amount - 1:
			program = args[i+1]
			flags.append("-")
			continue
		if arg in possible_flags:
			flags.append(arg)
	if not "-" in flags:
		path = input("Input the complete path to your maumivu.au file: ") if args_amount < 2 else args[1]
		file = open(path + ("" if path.endswith("maumivu.au") else "\maumivu.au"), "r")
		program = file.read()
		file.close()

	print("Program:")
	print(repr(program))

	valid_commands = [
		"\\[@",
		"@\\]",
		"\\[",
		"\\]",
		"\\$.",
		"\\\\.",
		"!",
		":\r?\n?",
		"\".*\"",
		"[ \t\f\v]",
		":$",
		"\$",
		"\^"
	]
	lexer = Lexer(program, valid_commands)
	validator = Validator()
	validator.run(deepcopy(lexer))
	parser = Parser()
	parser.run(deepcopy(lexer))
	i = AUI()
	i.run(deepcopy(lexer))