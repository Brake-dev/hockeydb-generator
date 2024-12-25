use rand::{self, thread_rng, Rng};

use crate::skaters::Nationality;

pub const FORENAME_CAN: [&str; 203] = [
    "Sidney",
    "Steven",
    "Claude",
    "John",
    "Connor",
    "Brad",
    "Nathan",
    "Jamie",
    "Corey",
    "Brent",
    "Matt",
    "Tyler",
    "Ryan",
    "David",
    "Kristopher",
    "Mark",
    "Jonathan",
    "Jordan",
    "Taylor",
    "Jeff",
    "Mitchell",
    "Nazem",
    "Brayden",
    "Alex",
    "Sam",
    "Sean",
    "Adam",
    "Reilly",
    "Bo",
    "Tyson",
    "Jaden",
    "Morgan",
    "Dougie",
    "Mathew",
    "Brendan",
    "Travis",
    "Max",
    "Zach",
    "Jared",
    "Cale",
    "Phillip",
    "Pierre-Luc",
    "Aaron",
    "T.J.",
    "Tom",
    "Josh",
    "Jean-Gabriel",
    "Dylan",
    "Robert",
    "Shea",
    "Yanni",
    "Nicholas",
    "Damon",
    "Anthony",
    "Marcus",
    "Tanner",
    "Chandler",
    "Jake",
    "Darnell",
    "Alexander",
    "Colton",
    "Thomas",
    "Carter",
    "Brandon",
    "Drake",
    "Vince",
    "Scott",
    "Mike",
    "Casey",
    "Devon",
    "Andreas",
    "Jakob",
    "Andrew",
    "Danton",
    "Evan",
    "Mackenzie",
    "Cody",
    "Samuel",
    "Noah",
    "Lawson",
    "Robby",
    "Brenden",
    "Luke",
    "Christopher",
    "Michael",
    "Nick",
    "Warren",
    "Matthew",
    "Colin",
    "Barclay",
    "Alexis",
    "Mason",
    "Seth",
    "Ben",
    "Owen",
    "Brock",
    "Jason",
    "Nicolas",
    "Dawson",
    "Calvin",
    "Mathieu",
    "Devin",
    "Gabriel",
    "Erik",
    "Frederick",
    "Wyatt",
    "Curtis",
    "Joel",
    "Troy",
    "William",
    "Brett",
    "Kirby",
    "Quinton",
    "Barrett",
    "Cole",
    "Bowen",
    "Logan",
    "Keegan",
    "Mario",
    "Dante",
    "Carson",
    "Kent",
    "Peyton",
    "Philip",
    "Jacob",
    "Alexandre",
    "Jack",
    "Joseph",
    "Braden",
    "Justin",
    "Ty",
    "Kaiden",
    "Joshua",
    "Jeremy",
    "Darren",
    "Will",
    "Haydn",
    "Parker",
    "Ridly",
    "Julien",
    "A.J.",
    "Zachary",
    "Pierre-Olivier",
    "Dennis",
    "Johnathan",
    "Philippe",
    "Conor",
    "Kevin",
    "Bobby",
    "Zack",
    "Jansen",
    "Liam",
    "Ross",
    "Kurtis",
    "Beck",
    "Hendrix",
    "Arber",
    "Tye",
    "Brandt",
    "Ryker",
    "Simon",
    "Givani",
    "Marc-Andre",
    "Shane",
    "Kyle",
    "Macklin",
    "Olen",
    "Vincent",
    "Reese",
    "Declan",
    "Jonah",
    "Kaedan",
    "Isaak",
    "Tristan",
    "Nolan",
    "Akil",
    "Cameron",
    "Jeffrey",
    "Mavrik",
    "Darcy",
    "Fraser",
    "Louis",
    "Maveric",
    "Isaiah",
    "Dustin",
    "Adin",
    "James",
    "Lucas",
    "Eric",
    "Stuart",
    "Ethan",
    "Calum",
    "Arshdeep",
    "Daemon",
    "Gage",
    "Trent",
    "Sebastian",
    "Yaniv",
    "Jett",
    "Keaton",
    "Jet",
    "Benjamin",
    "Spencer",
];

pub const SURNAME_CAN: [&str; 333] = [
    "Crosby",
    "Stamkos",
    "Giroux",
    "Tavares",
    "McDavid",
    "Marchand",
    "MacKinnon",
    "Benn",
    "Perry",
    "Burns",
    "Duchene",
    "Seguin",
    "O'Reilly",
    "Perron",
    "Letang",
    "Scheifele",
    "Huberdeau",
    "Nugent-Hopkins",
    "Eberle",
    "Hall",
    "Staal",
    "Skinner",
    "Marner",
    "Kadri",
    "Schenn",
    "Pietrangelo",
    "Stone",
    "Point",
    "Reinhart",
    "Monahan",
    "Toffoli",
    "Henrique",
    "Smith",
    "Horvat",
    "Killorn",
    "Couturier",
    "Barrie",
    "Marchessault",
    "Schwartz",
    "Rielly",
    "Hamilton",
    "Strome",
    "Barzal",
    "Gallagher",
    "Konecny",
    "Domi",
    "Hyman",
    "Spurgeon",
    "Myers",
    "Makar",
    "Danault",
    "McCann",
    "Dubois",
    "Ekblad",
    "Brodie",
    "Wilson",
    "Drouin",
    "Morrissey",
    "Pageau",
    "Thomas",
    "Theodore",
    "Gourde",
    "Suzuki",
    "Bennett",
    "Severson",
    "Duclair",
    "Foligno",
    "Mantha",
    "Pearson",
    "Stephenson",
    "DeBrusk",
    "Kyrou",
    "Nurse",
    "Kerfoot",
    "Parayko",
    "Bertuzzi",
    "Chabot",
    "Verhaeghe",
    "Montour",
    "Cirelli",
    "Lowry",
    "Batherson",
    "Beauvillier",
    "Dunn",
    "Laughton",
    "Matheson",
    "Dumba",
    "Cizikas",
    "Toews",
    "Athanasiou",
    "Hagel",
    "Brown",
    "Chychrun",
    "Hamonic",
    "Anderson",
    "Savard",
    "Mangiapane",
    "Heinen",
    "Rodrigues",
    "Weegar",
    "Ceci",
    "Girard",
    "Martinook",
    "Dobson",
    "Crouse",
    "Fabbri",
    "Dillon",
    "Sissons",
    "Sanheim",
    "Pulock",
    "Tanev",
    "Bunting",
    "Bouchard",
    "Cousins",
    "Cozens",
    "Foegele",
    "Martin",
    "Miller",
    "Paul",
    "Goodrow",
    "Lafrenière",
    "Marchment",
    "Jarvis",
    "Chiarot",
    "McNabb",
    "Manson",
    "Tippett",
    "McGinn",
    "Dickinson",
    "Demelo",
    "Roy",
    "Mercer",
    "Pelech",
    "Jost",
    "Joseph",
    "Shore",
    "Vilardi",
    "Reaves",
    "Oleksiak",
    "Gudbranson",
    "Gaudreau",
    "Steel",
    "Rasmussen",
    "Johnston",
    "Lazar",
    "Frost",
    "Hutton",
    "Amadio",
    "Edmundson",
    "Stecher",
    "Carrier",
    "Graves",
    "Jankowski",
    "Howden",
    "Geekie",
    "Kulak",
    "Dach",
    "Durzi",
    "Evans",
    "Newhook",
    "Deslauriers",
    "Walker",
    "Benning",
    "Byfield",
    "McTavish",
    "Hayton",
    "Perfetti",
    "Sillinger",
    "Caggiula",
    "McLeod",
    "Byram",
    "Power",
    "O'Connor",
    "Bedard",
    "Jeannot",
    "Raddysh",
    "Kolesar",
    "Aube-Kubel",
    "Ferraro",
    "Guenther",
    "Fabbro",
    "McMichael",
    "Glass",
    "Soucy",
    "Johnson",
    "Krebs",
    "Bortuzzo",
    "Tomasino",
    "Hague",
    "Walman",
    "Middleton",
    "Gregor",
    "McBain",
    "Neighbours",
    "Veleno",
    "Harley",
    "Evangelista",
    "Quinn",
    "Bastian",
    "Dermott",
    "Whitecloud",
    "Carrick",
    "Bean",
    "Schneider",
    "Drysdale",
    "Lomberg",
    "Danforth",
    "Foerster",
    "Zary",
    "Petrovic",
    "Barron",
    "Lorentz",
    "Leason",
    "Olivier",
    "Dellandrea",
    "Guhle",
    "Mahura",
    "Carcone",
    "Lauzon",
    "Cuylle",
    "Dewar",
    "Fleury",
    "Kelly",
    "Greig",
    "Gauthier",
    "Fantilli",
    "Greer",
    "Benson",
    "Spence",
    "Pezzetta",
    "Cholowski",
    "Kovacevic",
    "Bryson",
    "Holloway",
    "Timmins",
    "Bahl",
    "McMann",
    "MacEwen",
    "Harkins",
    "O'Brien",
    "MacDermid",
    "Stankoven",
    "Malenstyn",
    "Lapierre",
    "Hanley",
    "Xhekaj",
    "Kartye",
    "Stanley",
    "Sgarbossa",
    "Clarke",
    "Benoit",
    "Dowling",
    "Foudy",
    "Coghlan",
    "Wright",
    "Brazeau",
    "Burroughs",
    "Celebrini",
    "Bernard-Docker",
    "Bolduc",
    "Poitras",
    "Zellweger",
    "Barre-Boulet",
    "Desharnais",
    "Stephens",
    "Juulsen",
    "Chisholm",
    "Gadjovich",
    "Korchinski",
    "Richard",
    "Korczak",
    "Friedman",
    "Christiansen",
    "Pelletier",
    "Phillips",
    "Pachal",
    "Jarry",
    "McIlrath",
    "Wotherspoon",
    "Foote",
    "Kirkland",
    "L'Heureux",
    "Ludvig",
    "Binnington",
    "Talbot",
    "Schwindt",
    "Allen",
    "Truchon-Viel",
    "Thompson",
    "Bourque",
    "Allan",
    "Kuemper",
    "Minten",
    "Mailloux",
    "Crevier",
    "Luneau",
    "Lamoureux",
    "George",
    "Hofer",
    "Tokarski",
    "Ingram",
    "Pickard",
    "Hill",
    "Montembeault",
    "Reimer",
    "Condotta",
    "Reinhardt",
    "Pickering",
    "Rempe",
    "Poulin",
    "Ostapchuk",
    "Comrie",
    "Kozak",
    "Cardwell",
    "Ritchie",
    "Bains",
    "Philp",
    "Hunt",
    "Winterton",
    "Goncalves",
    "Blackwood",
    "Miner",
    "Cossa",
    "Milne",
    "Perets",
    "Légaré",
    "Luchanko",
    "Stienburg",
    "Greaves",
    "Bowers",
    "Jones",
    "Levi",
    "Wedgewood",
];

pub const FORENAME_USA: [&str; 141] = [
    "Patrick",
    "John",
    "Ryan",
    "Max",
    "Auston",
    "J.T.",
    "James",
    "Matthew",
    "Nick",
    "Chris",
    "Jack",
    "Brock",
    "Vincent",
    "Dylan",
    "Kyle",
    "Jake",
    "Brandon",
    "Cam",
    "Anders",
    "Alex",
    "Charlie",
    "Clayton",
    "Craig",
    "Justin",
    "Tyler",
    "Kevin",
    "Seth",
    "Bryan",
    "Shayne",
    "Jason",
    "Jeff",
    "Brady",
    "Quinn",
    "Erik",
    "Zachary",
    "Adam",
    "Jacob",
    "Noah",
    "Andrew",
    "Frank",
    "Alec",
    "Jaccob",
    "Tage",
    "Conor",
    "Blake",
    "Troy",
    "Neal",
    "Zach",
    "Trevor",
    "Nate",
    "Christian",
    "Casey",
    "Derek",
    "Brett",
    "Ian",
    "Joel",
    "Jimmy",
    "Miles",
    "Cole",
    "Luke",
    "Brian",
    "Nic",
    "Jordan",
    "Connor",
    "Garnet",
    "Matt",
    "Stefan",
    "Joshua",
    "Sean",
    "Sonny",
    "Ross",
    "Kailer",
    "Mason",
    "Mike",
    "Scott",
    "Austin",
    "Travis",
    "Noel",
    "Jon",
    "K'Andre",
    "Eric",
    "Trent",
    "Thomas",
    "Sam",
    "Colin",
    "Shane",
    "Kiefer",
    "Oliver",
    "Logan",
    "Michael",
    "Dakota",
    "Drew",
    "Paul",
    "Caleb",
    "William",
    "Nicklaus",
    "Jalen",
    "Chad",
    "Joey",
    "Jonny",
    "Bobby",
    "Hudson",
    "Mark",
    "Mattias",
    "Jackson",
    "Zac",
    "Steve",
    "Lane",
    "Henry",
    "Jonathan",
    "Walker",
    "Mitchell",
    "Jayden",
    "Dennis",
    "Cutter",
    "Ty",
    "Josh",
    "Wyatt",
    "Ben",
    "Brendan",
    "Corey",
    "Marc",
    "Victor",
    "Thatcher",
    "Seamus",
    "Anthony",
    "Riley",
    "Grant",
    "Alexander",
    "T.J.",
    "Dustin",
    "Cayden",
    "Spencer",
    "Pheonix",
    "Declan",
    "Callahan",
    "Jeremy",
    "Chase",
    "Rutger",
    "Jaxson",
    "Joseph",
];

pub const SURNAME_USA: [&str; 230] = [
    "Kane",
    "Carlson",
    "Suter",
    "Pacioretty",
    "Matthews",
    "Miller",
    "van Riemsdyk",
    "Tkachuk",
    "Foligno",
    "Kreider",
    "Eichel",
    "Nelson",
    "Trocheck",
    "Larkin",
    "Connor",
    "Guentzel",
    "Saad",
    "Palmieri",
    "Atkinson",
    "Lee",
    "DeBrincat",
    "Coyle",
    "Fowler",
    "Keller",
    "Smith",
    "Faulk",
    "Johnson",
    "Hayes",
    "Jones",
    "Leddy",
    "Boeser",
    "McDonagh",
    "Rust",
    "Gostisbehere",
    "Zucker",
    "Schmaltz",
    "Petry",
    "Hughes",
    "Tuch",
    "Robertson",
    "Werenski",
    "Fox",
    "Bjugstad",
    "Trouba",
    "Maroon",
    "Hanifin",
    "Copp",
    "McAvoy",
    "Vatrano",
    "Hartman",
    "Martinez",
    "Slavin",
    "Thompson",
    "Sheary",
    "Garland",
    "Coleman",
    "Compher",
    "Skjei",
    "Terry",
    "Iafallo",
    "Pionk",
    "Roslovic",
    "Labanc",
    "Bogosian",
    "Lewis",
    "Schmidt",
    "Dvorak",
    "Mittelstadt",
    "Ryan",
    "Nieto",
    "Pesce",
    "Cole",
    "Boldy",
    "Farabee",
    "Moore",
    "Vesey",
    "Donato",
    "Wood",
    "Caufield",
    "Zegras",
    "Glendening",
    "Dumoulin",
    "Dowd",
    "McCabe",
    "Greenway",
    "Murphy",
    "Hathaway",
    "Grzelcyk",
    "Noesen",
    "Jensen",
    "Norris",
    "Kuraly",
    "Milano",
    "van",
    "Kunin",
    "Colton",
    "Fischer",
    "Yamamoto",
    "Appleton",
    "Reilly",
    "Mayfield",
    "Beniers",
    "Watson",
    "Boyd",
    "Lizotte",
    "Acciari",
    "Roy",
    "Merrill",
    "Robinson",
    "Frederic",
    "Carlo",
    "Novak",
    "Forbort",
    "Motte",
    "Aston-Reese",
    "Holl",
    "Oesterle",
    "Gaudette",
    "Sanderson",
    "Lafferty",
    "Lindgren",
    "Blackwell",
    "Pinto",
    "Poehling",
    "Sherwood",
    "Cates",
    "Wahlstrom",
    "Cooley",
    "Anderson",
    "Joshua",
    "Clifton",
    "Wagner",
    "York",
    "Faber",
    "O'Connor",
    "Cotter",
    "Borgen",
    "Rooney",
    "Knies",
    "McCarron",
    "Perbix",
    "Seeler",
    "Chatfield",
    "Ruhwedel",
    "Peeke",
    "Duhaime",
    "Drury",
    "Laferriere",
    "Eyssimont",
    "Brodzinski",
    "Brink",
    "Kesselring",
    "Hayden",
    "Samberg",
    "Fasching",
    "Kastelic",
    "Vlasic",
    "Harris",
    "Samuelsson",
    "Perunovich",
    "LaCombe",
    "Brown",
    "Lohrei",
    "Santini",
    "Coronato",
    "Hutson",
    "Thrun",
    "Quick",
    "Duehr",
    "Blankenburg",
    "Beecher",
    "Chaffee",
    "Struble",
    "Turcotte",
    "Gilbert",
    "Gravel",
    "MacLean",
    "Malinski",
    "Koepke",
    "Gauthier",
    "Emberson",
    "Doan",
    "Mackey",
    "Kaiser",
    "Blake",
    "Kessel",
    "Samoskevich",
    "Hellebuyck",
    "Meyers",
    "Brisson",
    "Schueneman",
    "Del",
    "Gibson",
    "McLaughlin",
    "Helleson",
    "Sasson",
    "Mancini",
    "Hardman",
    "Kleven",
    "Oettinger",
    "Demko",
    "Casey",
    "Morelli",
    "Stolarz",
    "Berard",
    "Tufte",
    "Hutton",
    "Nedeljkovic",
    "Shea",
    "Tynan",
    "St.",
    "Wolf",
    "Primeau",
    "Knight",
    "Copley",
    "DeSmith",
    "Carlile",
    "Nazar",
    "Burke",
    "Colangelo",
    "Steeves",
    "Lyon",
    "Swayman",
    "Commesso",
    "Bradley",
    "McGroarty",
    "Stauber",
    "Giles",
    "Woll",
    "Daccord",
];

pub const FORENAME_SWE: [&str; 52] = [
    "Erik",
    "Victor",
    "Mika",
    "Filip",
    "Elias",
    "William",
    "Mikael",
    "Gustav",
    "Marcus",
    "Oliver",
    "Rickard",
    "Jesper",
    "Viktor",
    "Andre",
    "Adrian",
    "Alexander",
    "Mattias",
    "Hampus",
    "Rasmus",
    "Joel",
    "Jonas",
    "Adam",
    "Lucas",
    "Oscar",
    "Pierre",
    "Fabian",
    "Nils",
    "Isac",
    "Carl",
    "Timothy",
    "Robert",
    "Kevin",
    "Simon",
    "Jonatan",
    "Leo",
    "Pontus",
    "Philip",
    "Jacob",
    "Andreas",
    "David",
    "Emil",
    "Linus",
    "Liam",
    "André",
    "Anton",
    "Jonathan",
    "Helge",
    "Albert",
    "Dennis",
    "Isak",
    "Arvid",
    "Samuel",
];

pub const SURNAME_SWE: [&str; 72] = [
    "Karlsson",
    "Hedman",
    "Zibanejad",
    "Forsberg",
    "Lindholm",
    "Nylander",
    "Backlund",
    "Nyquist",
    "Johansson",
    "Ekman-Larsson",
    "Rakell",
    "Pettersson",
    "Bratt",
    "Arvidsson",
    "Burakovsky",
    "Kempe",
    "Wennberg",
    "Ekholm",
    "Dahlin",
    "Eriksson",
    "Brodin",
    "Gustafsson",
    "Larsson",
    "Andersson",
    "Janmark-Nylén",
    "Raymond",
    "Olofsson",
    "Forsling",
    "Sundqvist",
    "Engvall",
    "Sandin",
    "Zetterlund",
    "Höglander",
    "Boqvist",
    "Eklund",
    "Brännström",
    "Lundeström",
    "Grundström",
    "Liljegren",
    "Hägg",
    "Kylington",
    "Stenlund",
    "Holmström",
    "Holtz",
    "Lundkvist",
    "Berggren",
    "Carlsson",
    "Holmberg",
    "Åman",
    "Broberg",
    "Markström",
    "Englund",
    "Edvinsson",
    "Heineman",
    "Bäck",
    "Edström",
    "Andrae",
    "Ullmark",
    "Moverare",
    "Gustavsson",
    "Öhgren",
    "Wilsby",
    "Lee",
    "Lekkerimäki",
    "Grans",
    "Portillo",
    "Hildeby",
    "Wallstedt",
    "Rosén",
    "Högberg",
    "Söderblom",
    "Ersson",
];

pub const FORENAME_RUS: [&str; 38] = [
    "Alexander",
    "Evgeni",
    "Nikita",
    "Artemi",
    "Vladimir",
    "Pavel",
    "Kirill",
    "Andrei",
    "Evgeny",
    "Vladislav",
    "Dmitry",
    "Valeri",
    "Mikhail",
    "Ivan",
    "Dmitri",
    "Ilya",
    "Yakov",
    "Artyom",
    "Yegor",
    "Klim",
    "Alexei",
    "Vasili",
    "Matvei",
    "Daniil",
    "Maxim",
    "Nikolai",
    "Sergei",
    "Alexandar",
    "Semyon",
    "Marat",
    "Grigory",
    "Igor",
    "Fyodor",
    "Vasily",
    "Shakir",
    "Yaroslav",
    "Georgi",
    "Pyotr",
];

pub const SURNAME_RUS: [&str; 61] = [
    "Ovechkin",
    "Malkin",
    "Kucherov",
    "Panarin",
    "Tarasenko",
    "Buchnevich",
    "Kaprizov",
    "Svechnikov",
    "Dadonov",
    "Namestnikov",
    "Orlov",
    "Nichushkin",
    "Sergachev",
    "Barbashev",
    "Provorov",
    "Kulikov",
    "Zadorov",
    "Mikheyev",
    "Kuzmenko",
    "Gavrikov",
    "Marchenko",
    "Trenin",
    "Zub",
    "Chinakhov",
    "Romanov",
    "Lyubushkin",
    "Dorofeyev",
    "Kostin",
    "Toropchenko",
    "Voronkov",
    "Podkolzin",
    "Mintyukov",
    "Zamula",
    "Michkov",
    "Miromanov",
    "Vasilevski",
    "Tsyplakov",
    "Kovalenko",
    "Bobrovsky",
    "Miroshnichenko",
    "Alexeyev",
    "Georgiev",
    "Varlamov",
    "Khusnutdinov",
    "Denisenko",
    "Gushchin",
    "Chibrikov",
    "Samsonov",
    "Sorokin",
    "Shesterkin",
    "Svechkov",
    "Ponomaryov",
    "Tarasov",
    "Mukhamadullin",
    "Askarov",
    "Merkulov",
    "Kochetkov",
    "Misyul",
    "Grebyonkin",
    "Prishchepov",
    "Fedotov",
];

pub const FORENAME_FIN: [&str; 38] = [
    "Aleksander",
    "Mikko",
    "Sebastian",
    "Mikael",
    "Teuvo",
    "Patrik",
    "Roope",
    "Erik",
    "Rasmus",
    "Miro",
    "Artturi",
    "Kasperi",
    "Esa",
    "Joel",
    "Olli",
    "Jesperi",
    "Eeli",
    "Kaapo",
    "Anton",
    "Jesse",
    "Matias",
    "Eetu",
    "Henri",
    "Juuso",
    "Niko",
    "Jani",
    "Urho",
    "Valtteri",
    "Ville",
    "Juuse",
    "Aatu",
    "Kevin",
    "Ukko-Pekka",
    "Joonas",
    "Oliver",
    "Samuel",
    "Brad",
    "Justus",
];

pub const SURNAME_FIN: [&str; 44] = [
    "Barkov",
    "Rantanen",
    "Aho",
    "Granlund",
    "Teräväinen",
    "Laine",
    "Hintz",
    "Haula",
    "Ristolainen",
    "Heiskanen",
    "Lehkonen",
    "Kapanen",
    "Lindell",
    "Armia",
    "Määttä",
    "Kotkaniemi",
    "Tolvanen",
    "Kakko",
    "Lundell",
    "Puljujärvi",
    "Maccelli",
    "Luostarinen",
    "Jokiharju",
    "Välimäki",
    "Mikkola",
    "Hakanpää",
    "Kiviranta",
    "Pärssinen",
    "Kupari",
    "Vaakanainen",
    "Puustinen",
    "Heinola",
    "Saros",
    "Räty",
    "Pyyhtiä",
    "Lankinen",
    "Luukkonen",
    "Korpisalo",
    "Helenius",
    "Kähkönen",
    "Lambert",
    "Annunen",
    "Husso",
    "Blomqvist",
];

pub const FORENAME_CZE: [&str; 19] = [
    "David", "Tomas", "Ondrej", "Pavel", "Martin", "Jakub", "Filip", "Radek", "Radko", "Jan",
    "Ivan", "Vitek", "Petr", "Jiri", "Daniel", "Karel", "Matej", "Adam", "Lukas",
];

pub const SURNAME_CZE: [&str; 25] = [
    "Pastrnak", "Hertl", "Palat", "Zacha", "Necas", "Vrana", "Hronek", "Faksa", "Gudas", "Chytil",
    "Kämpf", "Nosek", "Rutta", "Lauko", "Jiricek", "Ivan", "Vanecek", "Mrazek", "Kulich", "Vladar",
    "Vejmelka", "Blümel", "Klapka", "Rittich", "Dostal",
];

pub fn get_name_by_nationality(nationality: &Nationality) -> String {
    match nationality {
        Nationality::CAN => {
            let fore_index = thread_rng().gen_range(0..FORENAME_CAN.len());
            let sur_index = thread_rng().gen_range(0..SURNAME_CAN.len());

            return String::from(FORENAME_CAN[fore_index]) + " " + SURNAME_CAN[sur_index];
        }
        Nationality::USA => {
            let fore_index = thread_rng().gen_range(0..FORENAME_USA.len());
            let sur_index = thread_rng().gen_range(0..SURNAME_USA.len());

            return String::from(FORENAME_USA[fore_index]) + " " + SURNAME_USA[sur_index];
        }
        Nationality::SWE => {
            let fore_index = thread_rng().gen_range(0..FORENAME_SWE.len());
            let sur_index = thread_rng().gen_range(0..SURNAME_SWE.len());

            return String::from(FORENAME_SWE[fore_index]) + " " + SURNAME_SWE[sur_index];
        }
        Nationality::RUS => {
            let fore_index = thread_rng().gen_range(0..FORENAME_RUS.len());
            let sur_index = thread_rng().gen_range(0..SURNAME_RUS.len());

            return String::from(FORENAME_RUS[fore_index]) + " " + SURNAME_RUS[sur_index];
        }
        Nationality::FIN => {
            let fore_index = thread_rng().gen_range(0..FORENAME_FIN.len());
            let sur_index = thread_rng().gen_range(0..SURNAME_FIN.len());

            return String::from(FORENAME_FIN[fore_index]) + " " + SURNAME_FIN[sur_index];
        }
        Nationality::CZE => {
            let fore_index = thread_rng().gen_range(0..FORENAME_CZE.len());
            let sur_index = thread_rng().gen_range(0..SURNAME_CZE.len());

            return String::from(FORENAME_CZE[fore_index]) + " " + SURNAME_CZE[sur_index];
        }
    }
}