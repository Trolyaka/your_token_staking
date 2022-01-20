import express from "express";
import path from "path";
const app = express();
const port = 5000;

app.use(express.json())

app.get('/v1/public/health', (_, res) => res.json({status: 'ok'}))

app.post("/v1/public/contract_call",(req, res)=>{

    res.json("Contract called successful!")
})

app.get("/v1/public/contract_call",(req, res)=>{
    res.sendFile(path.join(__dirname, '/template/index.html'));
});

const main = async () => {
    try {
        app.listen(port, () => console.log(`Running on port ${port}`));
    } catch (e) {
        console.log(e)
    }
}

main()
