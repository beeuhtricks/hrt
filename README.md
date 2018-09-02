# hrt
calculate how long you've been on hormone replacement therapy

this currently works for rfc 2822 formatted dates, such as "Thu, 6 Jul 2017 12:00:00 -0400" where "-0400" is the exact utc offset for eastern daylight time, because i started hrt in boston in the summer. there is currently no error handling and all "Result" types are just unwrapped, because this was hacked together quickly. the first time a date is input it will write it to the file hrt/hrt.txt in your home directory, after which a date need not be specified and the duration will be calculated against the current time.
