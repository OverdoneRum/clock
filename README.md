clock is a CLI tool to fetch local times for different timezones (cities)

This project is for personal use and is not intended for use in other projects, thus I will be treating this readme more as a journal entry than documentation.

Why?

This tool is a part of my journey to implement small QOL changes in my daily life through code. In my day job I work as a software consultant. I don't write any code, however, I still enjoy developing the odd side project and leaning new things. As a part of my role I work across many timezones and have a need to schedule meetings with clients or other consultants. Sometimes I won't have my calendar open in front of me, and much prefer to use keyboard shortcuts to speed things up. So, I had the thought to develop a CLI tool to achieve this. Whilst I will typically choose to develop in C#, as it is my preferred language, I have been learning the fundamentals of Rust and decided this would be a great program to learn the ropes.

The fundamental idea of this program was to be able to invoke it from the command line with a 3-letter code for the city/country I wanted to know the time of. I also wanted to account for daylight savings timezone differences, for this reason I decided to go with an already existing crate rather than reinventing the wheel. Using chrono and chrono_tz I was able to create preset variables for the places I needed. At present I only work with clients and consultants in a handful of timezones, so there was no real need to dynamically lookup these values, and rather I could just set them as variables, chrono_tz was really good with this, having predefined modules for each country. 

Once I had all my variables, all I needed was to implement passing arguments form the commandline into the app, which was relatively easy to do with the std module (this also taught me how vectors work in rust), and to create a match statement (similar to a case statement) to match the input to its corresponding variables.

Whilst I usually memorise these times across each timezone, I now will no longer need to remember when daylight svaings changes across each timezone, which will surely save me a goiogle search or two.
