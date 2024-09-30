import path from "path";
import {getLlama, LlamaChatSession} from "node-llama-cpp";

const llama = await getLlama();
const model = await llama.loadModel({
    modelPath: path.join("/model", "meta-llama-3.1-8b-instruct-q4_k_m.gguf")
});
const context = await model.createContext();
const session = new LlamaChatSession({
    contextSequence: context.getSequence()
});


const usercode = async (data) => {

    const q1 = "Hi there, how are you?";
    console.log("User: " + q1);

    const a1 = await session.prompt(q1);
    console.log("AI: " + a1);


    const q2 = "Summarize what you said";
    console.log("User: " + q2);

    const a2 = await session.prompt(q2);
    console.log("AI: " + a2);


    data["js"] = "Hello, world!"
    return data
};


export default usercode;



// module.exports = async (data) => {
//     data["js"] = "Hello, world!"
//     return data
// };
