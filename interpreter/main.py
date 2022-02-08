import sys
from copy import deepcopy
import os
import re

class Lexer:
	def __init__(self, text, rules, file):
		self.text = text
		self.rules = rules
		self.line = 1
		self.column = 1
		self.pos = 0
		self.comment = False
		self.file = file
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
		raise_error(("Syntax error at %d:%d in %s" % (self.line, self.column, self.file)), 1)

class Validator:
	def run(self, lex: Lexer):
		p = None
		t = lex.next()
		while t:
			p = t
			t = lex.next()
		if not re.match(":\r?\n?", p[0]):
			raise_error(("Syntax error at %d:%d in %s - ':' expected" % (lex.line, lex.column, lex.file)), 1)

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
			raise_error(("Syntax error at %d:%d in %s - ':' expected" % (lex.line, lex.column, lex.file)), 1)

class Runner:
	def __init__(self, root_path):
		self.root_path = root_path
		self.valid_commands = [
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

	def run_file(self, file_path):
		file = open(os.path.join(path, file_path), "r")
		program = file.read()
		file.close()
		self.run(program, file_path)

	def run_user_input(self, program):
		self.run(program, "<input_main>")

	def run(self, program, file):
		print(repr(program))
		lexer = Lexer(program, self.valid_commands, file)
		validator = Validator()
		validator.run(deepcopy(lexer))
		parser = Parser()
		parser.run(deepcopy(lexer))
		commands = parser.commands

class Warner:
	def __init__(self, flags):
		self.disabled = []
		for flag in flags:
			flag = flag.lower()
			if flag == "--disable-warnings":
				self.disabled.append("all")
			elif flag == "--disable-path-warning":
				self.disabled.append("path")

	def warn(self, warning_type):
		if "all" in self.disabled or warning_type in self.disabled:
			return
		if warning_type == "path":
			print("Warning: No code path supplied, this will make it impossible to run files from the code (you can use the --disable-warnings flag to disable all warnings or --disable-path-warning to disable this particular warning")
			return

def raise_error(text, code = 1):
		print(text)
		sys.exit(code)

if __name__ == "__main__":
	args = sys.argv
	debug_heavy = False
	flags = []
	possible_flags = [
		"--debug",
		"-",
		"--disable-warnings",
		"--disable-path-warning"
	]
	args_amount = len(args)
	path = None
	program = None
	for i in range(0, args_amount):
		arg = args[i]
		if arg == "-" and not "-" in flags and i < args_amount - 1:
			program = args[i+1]
			flags.append("-")
			continue
		if arg in possible_flags:
			flags.append(arg)
	if not "-" in flags:
		path: str = input("Input the complete path to your maumivu.au file: ") if args_amount < 2 else args[1]
		if path.endswith("maumivu.au"):
			path = path[0:-10]

	warner = Warner(flags)
	if path == None:
		warner.warn("path")

	runner = Runner(path)
	if program == None:
		runner.run_file("maumivu.au")
	else:
		runner.run_user_input(program)