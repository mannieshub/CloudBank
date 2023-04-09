import Head from "next/head";
import Image from "next/image";
import { Inter } from "next/font/google";
import styles from "@/styles/Home.module.css";
import { Card, Button } from "react-bootstrap";
import Form from "react-bootstrap/Form";
import InputGroup from "react-bootstrap/InputGroup";
import { useRouter } from "next/router";
import Container from "react-bootstrap/Container";
import Navbar from "react-bootstrap/Navbar";
import Dropdown from "react-bootstrap/Dropdown";
import DropdownButton from "react-bootstrap/DropdownButton";
import { useEffect, useState } from "react";

const inter = Inter({ subsets: ["latin"] });
const userId = 1212312121;
export default function Home() {
  const router = useRouter();
  
fetch(`http://localhost:8080/account/${userId}`)
  .then(response => response.json())
  .then(data => {
    const account = data.data[0];
    const username = account.account_name;
    document.getElementById('dropdown-menu-align-end').textContent = username;
    });

    const [result, setResult] = useState(null);
    const [error, setError] = useState(null);
    const [inputValue, setInputValue] = useState('');

    const handleDeposit = async () => {
      try {
        const response = await fetch(`http://localhost:8080/deposit/${userId}`, {
          method: 'POST', 
          headers: {
            'content-type': 'application/json',
            'Access-Control-Allow-Origin': 'http://localhost:8080'
          },
          body: JSON.stringify({
            amount: parseInt(inputValue)
          })
        });
    
        if (!response.ok) {
          throw new Error('Deposit failed');
        }
    
        const data = await response.json();
        const depositInfo = data.data[0];
        const message = data.message;
        setResult(
          <div className="alert alert-success mt-3" role="alert">
            {message} Your new balance is{" "}
            <strong>{depositInfo.available_balance}</strong> as of{" "}
            <strong>{depositInfo.time}</strong>
          </div>
        );
      
        setError(null);
      } catch (error) {
        setResult(null);
        setError(error.message);
      }
    };
    
    
    

  const handleChange = (event) => {
    setInputValue(event.target.value);
  };


  return (
    <div>
      <Head>
        <title>Web Bank</title>
        <meta name="description" content="Generated by create next app" />
        <meta name="viewport" content="width=device-width, initial-scale=1" />
        <link rel="icon" href="/favicon.ico" />
      </Head>
      <main>
        <div className={styles.menu}>
          <div className={styles.top} style={{ fontSize: "30px" }}>
            <div>Webbank</div>
          </div>
          <div style={{ justifyItem: "right" }}>
            <DropdownButton
              align="end"
              title="Username"
              id="dropdown-menu-align-end"
              variant="success"
            >
              <Dropdown.Item
                eventKey="1"
                style={{
                  cursor: "pointer",
                }}
                onClick={() => {
                  router.push("/menu");
                }}
              >
                Menu
              </Dropdown.Item>

              <Dropdown.Divider />
              <Dropdown.Item
                eventKey="4"
                style={{
                  cursor: "pointer",
                }}
                onClick={() => {
                  router.push("/");
                }}
              >
                Sign Out
              </Dropdown.Item>
            </DropdownButton>
          </div>
        </div>
        <div className={styles.card}>
          <Card style={{ width: "30rem" }}>
            <Card.Body>
              <Card.Title style={{ textAlign: "center" }}>ฝากเงิน</Card.Title>
              <div>
                <Form.Label htmlFor="basic-url">
                  จำนวนเงินที่ต้องการฝาก
                </Form.Label>
                <InputGroup className="mb-3">
                  <Form.Control
                    id="basic-url"
                    aria-describedby="basic-addon3"
                    value={inputValue}
                    onChange={handleChange}
                  />
                </InputGroup>
              </div>
              <Button onClick={handleDeposit} variant="success" style={{ width: "100%" }}>
                Confirm
              </Button>
              {result && <p>{result}</p>}
              {error && <p>Error: {error}</p>}
            </Card.Body>
          </Card>
        </div>
      </main>
    </div>
  );
}