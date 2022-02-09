from math import floor
from random import random
import sys
from copy import deepcopy
import os
import re

class BracketsMatcher:
	def __init__(self):
		self.brackets = {
			"while": {},
			"do_while": {},
			"while_local": {},
			"do_while_local": {}
		}
		self.brackets_mem = []
		self.bracket_keys = {
			"[": "while",
			"]": "while",
			"[@": "do_while",
			"@]": "do_while",
			"'[": "while_local",
			"']": "while_local",
			"'[@": "do_while_local",
			"'@]": "do_while_local"
		}
		self.ending_brackets_keys = {
			"while": "]",
			"do_while": "@]",
			"while_local": "']",
			"do_while_local": "'@]"
		}

	def match(self, code):
		for i in range(len(code)):
			char = code[i]
			if not char in ["[", "]", "[@", "@]", "'[", "']", "'[@", "'@]"]:
				continue
			if char in ["[", "[@", "'[", "'[@"]:
				self.brackets_mem.append([char, i, 0])
			for key in range(len(self.brackets_mem)):
				self.brackets_mem[key][2] += self.num_equals(self.brackets_mem[key][0], char)
				wanted_end = self.ending_brackets_keys[self.bracket_keys[self.brackets_mem[key][0]]]
				if self.brackets_mem[key][2] == 0 and char == wanted_end:
					cat = self.bracket_keys[wanted_end]
					self.brackets[cat][self.brackets_mem[key][1]] = i
					self.brackets[cat][i] = self.brackets_mem[key][1]
					self.brackets_mem.pop(key)

			
	def num_equals(self, left, right):
		if self.bracket_keys[left] != self.bracket_keys[right]:
			return 0
		if left == right:
			return 1
		return -1

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
				command = m.group(0)
				self.pos += len(command)
				if '\n' in command:
					self.line += command.count('\n')
					self.column += len(command.rsplit('\n', 1)[-1])
				else:
					self.column += len(command)
				return (command, self.line, self.column, self.file)
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
		self.commands_info = []

	def run(self, lex: Lexer):
		t = lex.next()
		while t:
			print(t)
			if '"' in t[0] or ":" in t[0]:
				t = lex.next()
				continue
			self.commands_info.append(t)
			self.commands.append(t[0])
			t = lex.next()

class Runner:
	def __init__(self, root_path, warner):
		self.root_path = root_path
		self.valid_commands = [
			# (\|[0-9]*\|)? lets you do |x|<command>, which will execute the command x times (leaving it empty will execute it <active cell value>.floor() times)
			"'?(\|[0-9]*\|)?!", # increment
			"'?(\|[0-9]*\|)?~", # decrement
			"'?(\|[0-9]*\|)?\+", # add
			"'?(\|[0-9]*\|)?\-", # subtract
			"'?(\|[0-9]*\|)?\*", # multiply
			"'?(\|[0-9]*\|)?\/", # divide
			"'?`", # generate a random number from 0 (inclusive) to 1 (exclusive)
			"'?(\|[0-9]*\|)?\>", # move right
			"'?(\|[0-9]*\|)?\<", # move left
			"'?\_", # floor
			"'?\&", # ceil
			"'?\^", # switch active memory
			"'?\\[@", # do-while start
			"'?@\\]", # do-while end
			"'?\\[", # while start
			"'?\\]", # while end
			"'?\\$.", # input number
			"'?\$\,", # input string
			"'?\\\\.", # output number
			"'?\\\\,", # output string
			"'?\?\=", # if the active memory cell = the not active memory cell, break
			"'?\?\<", # if the active memory cell < the not active memory cell, break
			"'?\?\>", # if the active memory cell > the not active memory cell, break
			";", # switch value of the active local memory cell and global memory cell
			":\r?\n?", # end of line
			":$", # end of line with any character after
			"\"[^\"]*\"", # comments
			"[ \t\f\v]" # whitespace
		]
		self.opposite_commands = {
			"!": "~",
			"~": "!",
			"+": "-",
			"-": "+",
			"*": "/",
			"/": "*",
			">": "<",
			"<": ">"
		}
		self.commands = []
		self.commands_info = []
		self.brackets = {}
		self.memory = [[0.0], [0.0]]
		self.pointers_mem = [0, 0]
		self.active_mem = 0
		self.program_pointer = 0
		self.loops = []
		self.warner = warner

	def run_file(self, file_path):
		file = open(os.path.join(path, file_path), "r")
		program = file.read()
		file.close()
		self.run(program, file_path)

	def run_user_input(self, program):
		self.run(program, "<input_main>")

	def run(self, program, file):
		local_memory = [[0.0], [0.0]]
		local_pointers_mem = [0, 0]
		local_active_mem = 0
		print("Program:")
		print(repr(program))
		lexer = Lexer(program, self.valid_commands, file)
		validator = Validator()
		validator.run(deepcopy(lexer))
		parser = Parser()
		parser.run(deepcopy(lexer))
		self.commands = parser.commands
		self.commands_info = parser.commands_info
		print("Commands:")
		print(self.commands)
		brackets_matcher = BracketsMatcher()
		brackets_matcher.match(self.commands)
		brackets_holder = brackets_matcher.brackets
		for loop_type in brackets_holder:
			for key in brackets_holder[loop_type]:
				self.brackets[key] = brackets_holder[loop_type][key]
		while self.program_pointer < len(self.commands):
			command = self.commands[self.program_pointer]
			(local_memory, local_pointers_mem, local_active_mem) = self.eval_command(command, local_memory, local_pointers_mem, local_active_mem)
		print("\nMain memory:")
		print(self.memory)
		print("Local memory:")
		print(local_memory)

	def eval_command(self, command: str, local_memory, local_pointers_mem, local_active_mem):
		is_local = command.startswith("'")
		if is_local:
			command = command[1:]
		# print(command, command.count("|"))
		(main_mem, main_mem_ptr, main_act) = (local_memory, local_pointers_mem, local_active_mem) if is_local else (self.memory, self.pointers_mem, self.active_mem)
		(loc_mem, loc_mem_ptr, loc_act) = (self.memory, self.pointers_mem, self.active_mem) if is_local else (local_memory, local_pointers_mem, local_active_mem)

		repeat = 1
		if command.count("|") == 2:
			_, num, command = command.split("|")
			if num == "":
				num = main_mem[main_act][main_mem_ptr[main_act]]
			num = int(num)
			if num < 0:
				num = abs(num)
				command = self.opposite_commands[command]
			repeat = num
		for _ in range(0, repeat):
			if command == "!":
				main_mem[main_act][main_mem_ptr[main_act]] += 1
			if command == "~":
				main_mem[main_act][main_mem_ptr[main_act]] -= 1
			if command == "+":
				main_mem[main_act][main_mem_ptr[main_act]] += main_mem[abs(main_act-1)][main_mem_ptr[abs(main_act-1)]]
			if command == "-":
				main_mem[main_act][main_mem_ptr[main_act]] -= main_mem[abs(main_act-1)][main_mem_ptr[abs(main_act-1)]]
			if command == "*":
				main_mem[main_act][main_mem_ptr[main_act]] *= main_mem[abs(main_act-1)][main_mem_ptr[abs(main_act-1)]]
			if command == "/":
				main_mem[main_act][main_mem_ptr[main_act]] /= main_mem[abs(main_act-1)][main_mem_ptr[abs(main_act-1)]]
			if command == ">":
				main_mem_ptr[main_act] += 1
				if main_mem_ptr[main_act] >= len(main_mem[main_act]):
					main_mem[main_act].append(0.0)
			if command == "<":
				main_mem_ptr[main_act] -= 1
				if main_mem_ptr[main_act] < 0:
					main_mem[main_act].insert(0, 0.0)
					self.warner.warn("too-left-pointer")
			if command == "^":
				main_act = abs(main_act-1)
			if command == "\\.":
				print(main_mem[main_act][main_mem_ptr[main_act]], end = '')
			if command == "\\,":
				print(chr(floor(main_mem[main_act][main_mem_ptr[main_act]])), end = '')
			if command == "$.":
				main_mem[main_act][main_mem_ptr[main_act]] = float(input())
			if command == "$,":
				main_mem[main_act][main_mem_ptr[main_act]] = ord(input()[0])
			if command == "[":
				if main_mem[main_act][main_mem_ptr[main_act]] == 0:
					if self.program_pointer in self.loops:
						self.loops.remove(self.program_pointer)
					self.program_pointer = self.brackets[self.program_pointer]
				elif not self.program_pointer in self.loops:
					self.loops.append(self.program_pointer)
			if command == "]":
				if main_mem[main_act][main_mem_ptr[main_act]] == 0:
					if self.program_pointer in self.loops:
						self.loops.remove(self.program_pointer)
				else:
					self.program_pointer = self.brackets[self.program_pointer]
			if command == "[@":
				if main_mem[main_act][main_mem_ptr[main_act]] == 0 and self.program_pointer in self.loops:
					if self.program_pointer in self.loops:
						self.loops.remove(self.program_pointer)
					self.program_pointer = self.brackets[self.program_pointer]
				elif not self.program_pointer in self.loops:
					self.loops.append(self.program_pointer)
			if command == "@]":
				if main_mem[main_act][main_mem_ptr[main_act]] == 0:
					if self.program_pointer in self.loops:
						self.loops.remove(self.program_pointer)
				else:
					self.program_pointer = self.brackets[self.program_pointer]
			if command == "`":
				main_mem[main_act][main_mem_ptr[main_act]] = random()

		self.program_pointer += 1
		self.memory = loc_mem if is_local else main_mem
		self.pointers_mem = loc_mem_ptr if is_local else main_mem_ptr
		self.active_mem = loc_act if is_local else main_act
		return (main_mem, main_mem_ptr, main_act) if is_local else (loc_mem, loc_mem_ptr, loc_act)

class Warner:
	def __init__(self, flags):
		self.disabled = []
		for flag in flags:
			flag = flag.lower()
			if flag == "--disable-warnings":
				self.disabled.append("all")
			elif flag == "--disable-path-warning":
				self.disabled.append("path")
			elif flag == "--disable-too-left-pointer-warning":
				self.disabled.append("too-left-pointer")

	def warn(self, warning_type):
		if "all" in self.disabled or warning_type in self.disabled:
			return
		if warning_type == "path":
			print("Warning: No code path supplied, this will make it impossible to run files from the code (you can use the --disable-warnings flag to disable all warnings or --disable-path-warning to disable this particular warning)")
			return
		if warning_type == "too-left-pointer":
			print("You moved to the -1 index in memory. This will not crash the program, but should generally be avoided (you can use the --disable-warnings flag to disable all warnings or --disable-too-left-pointer-warning to disable this particular warning)")

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
		"--disable-path-warning",
		"--disable-too-left-pointer-warning"
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

	runner = Runner(path, warner)
	if program == None:
		runner.run_file("maumivu.au")
	else:
		runner.run_user_input(program)