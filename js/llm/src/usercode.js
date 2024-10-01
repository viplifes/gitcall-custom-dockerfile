import {getLlama, LlamaChatSession} from "node-llama-cpp";

const llama = await getLlama();
const model = await llama.loadModel({
    modelPath: './meta-llama-3.1-8b-instruct-q4_k_m.gguf'
});
const sessionLifeTime = 1800; // 30min
const sessions = {};

console.log("llama.GPU: " + llama._gpu);

const usercode = async (data) => {

    const session = await getOrCreateSession(data.sessionId);
    console.log(`[${data.sessionId}] User: ${data.text}`);
    const answer = await session.prompt(data.text);
    console.log(`[${data.sessionId}] AI: ${answer}`);

    data["answer"] = answer
    return data
};


const getOrCreateSession = async (sessionId) => {
    // clean timer
    const cleanRef = setTimeout(() => {
        if (sessions[sessionId]){
            clearTimeout(sessions[sessionId].cleanRef);
            delete sessions[sessionId];
        }
    }, sessionLifeTime * 1000);

    // session exists
    if (sessions[sessionId]){
        clearTimeout(sessions[sessionId].cleanRef);
        sessions[sessionId].cleanRef = cleanRef;
        return sessions[sessionId].session;
    }

    // session not exists
    const context = await model.createContext();
    const session = new LlamaChatSession({
        contextSequence: context.getSequence()
    });
    sessions[sessionId] = {
        session: session,
        cleanRef: cleanRef,
    };
    
    return session
};


export default usercode;