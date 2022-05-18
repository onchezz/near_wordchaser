use serde_json::json;

//this function creates  initial random data for  the wordchaser game
//this immitates a simple rest api for json respond
pub fn json_data() -> serde_json::Value {
  let data = json!(
    [
  {
  "word": "consider",
  "meaning":"deem to be",
  "example":"At the moment, artemisinin-based therapies are considered the best treatment, but cost about $10 per dose - far too much for impoverished communities"
  },
  {
  "word": "minute",
  "meaning":"infinitely or immeasurably small",
  "example":"The minute stain on the document was not visible to the naked eye."
  },
  {
  "word":"accord",
  "meaning":"concurrence of opinion",
  "example":"The committee worked in accord on the bill, and it eventually passed."
  },
  {
  "word":"evident",
  "meaning":"clearly revealed to the mind or the senses or judgment",
  "example":"That confidence was certainly evident in the way Smith handled the winning play with 14 seconds left on the clock."
  },
  {
  "word":"practice",
  "meaning":"a customary way of operation or behavior",
  "example":"He directed and acted in plays every season and became known for exploring Elizabethan theatre practices."
   },
  {
  "word":"intend",
  "meaning":"have in mind as a purpose",
  "example":"Lipstick, as a product intended for topical use with limited absorption, is ingested only in very small quantities,” the agency said on its website."
   },
  {
  "word":"concern",
  "meaning":"something that interests you because it is important",
  "example":"The scandal broke out in October after former chief executive Michael Woodford claimed he was fired for raising concerns about the company's accounting practices."
  },
  {
  "word":"commit",
  "meaning":"perform an act, usually with a negative connotation",
  "example":"In an unprecedented front page article in 2003 The Times reported that Mr. Blair, a young reporter on its staff, had committed journalistic fraud."
   },
   {
  "word":"issue",
  "meaning":"some situation or event that is thought about",
  "example":"As a result, the privacy issues surrounding mobile computing are becoming ever-more complex."
   },
   {
  "word":"approach",
  "meaning":"move towards",
  "example":"Spain’s jobless rate for people ages 16 to 24 is approaching 50 percent."
  } ,
  {
  "word":"establish",
  "meaning":"set up or found",
  "example":"A small French colony, Port Louis, was established on East Falkland in 1764 and handed to the Spanish three years later."
   },
   {
  "word":"utter",
  "meaning":"without qualification",
  "example":"No one can blame an honest mechanic for holding a wealthy snob in utter contempt.Ingersoll, Robert Green"
   },
   {
  "word":"conduct",
  "meaning":"direct the course of; manage or control",
  "example":"Scientists have been conducting studies of individual genes for years"
   },
   {
  "word":"engage",
  "meaning":"consume all of one's attention or time",
  "example":"We had nearly two hundred passengers, who were seated about on the sofas, reading, or playing games, or engaged in conversation.Field, Henry M."
   },
   {
  "word":"obtain",
  "meaning":"come into possession of",
  "example":"He delayed making the unclassified report public while awaiting an Army review, but Rolling Stone magazine obtained the report and posted it Friday night.N"
  },
   {
  "word":"scarce",
  "meaning":"deficient in quantity or number compared with the demand",
  "example":"Meanwhile, heating oil could grow more scarce in the Northeast this winter, the Energy Department warned last month"}
  ,
  {
  "word":"policy",
  "meaning":"a plan of action adopted by an individual or social group",
  "example":"Inflation has lagged behind the central bank’s 2 percent target, giving policy makers extra scope to cut rates."
  },
  {
  "word":"straight",
  "meaning":"successive, without a break",
  "example":"After three straight losing seasons, Hoosiers fans were just hoping for a winning record."
  },
  {
  "word":"stock",
  "meaning":"capital raised by a corporation through the issue of shares",
  "example":"In other words, Apple’s stock is cheap, and you should buy it."},
  {
  "word":"apparent",
  "meaning":"clearly revealed to the mind or the senses or judgment",
  "example":"But the elderly creak is beginning to become apparent in McCartney’s voice."
  },
  {
  "word":"hello",
  "meaning":"simple greeting used daily by people",
  "example":"hello am Emanuael from starbucks how may i help you today??"
  }
  ]
     );
  data
}
