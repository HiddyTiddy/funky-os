#!/usr/bin/env python3

"""
println("\x1b[{code}m") does not work bc rust processes the thing not all at once
"""

tests = ["30m", "90m", "31m", "91m", "32m", "92m", "33m", "93m", "34m",
"94m", "35m", "95m", "36m", "96m", "37m", "97m",
]

test_str = "..."
s ="".join([f"{i}m{(5-3+(len(test_str))) * ' '}" for i in range(40,48)])
print(f'println!("       {s}");')
for test in tests:
    print(f'println!("{test}  \\x1b[{test} ', end='')
    for i in range(40,48):
            print(f"\\x1b[{test}\\x1b[{i}m {test_str} \\x1b[0m   ",end='')
    print('");')
