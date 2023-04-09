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
import Dropdown from 'react-bootstrap/Dropdown';
import DropdownButton from 'react-bootstrap/DropdownButton';
export default function register() {
  const router = useRouter();
  return (
    <div>
      <div>
        <Navbar style={{ backgroundColor:"#2A3B42" }}>
          <Navbar.Brand
            style={{
              paddingLeft: "50px",
              fontSize: "30px",
              fontWeight: "bold",
              color: "#45C2B2"
            }}
            href="#home"
          >
            Webbank
          </Navbar.Brand >
          <Navbar.Toggle />
          <Navbar.Collapse className="justify-content-end">
            <Navbar.Text className={styles.nsavsignout}>
              <button
                onClick={() => {
                  router.push("/");
                }}
                className={styles.btn}
              >
                Login
              </button>
            </Navbar.Text>
          </Navbar.Collapse>
        </Navbar>
      </div>
      <div style={{ marginTop: "50px" }}>
        <div className={styles.card}>
          <Card style={{ width: "30rem" }}>
            <Card.Body>
              <Card.Title style={{ textAlign: "center" }}>Register</Card.Title>
              <div>
                <div>
                <DropdownButton id="dropdown-basic-button" title="Deposit" variant="secondary">
      <Dropdown.Item href="#/action-1">Savings Account</Dropdown.Item>
      <Dropdown.Item href="#/action-2">Fixed deposit account</Dropdown.Item>
      <Dropdown.Item href="#/action-3">Current account</Dropdown.Item>
    </DropdownButton>
                </div>
                <Form.Label htmlFor="basic-url">Username</Form.Label>
                <InputGroup className="mb-3">
                  <Form.Control
                    id="basic-url"
                    aria-describedby="basic-addon3"
                  />
                </InputGroup>
                <Form.Label htmlFor="basic-url">Password</Form.Label>
                <InputGroup className="mb-4">
                  <Form.Control
                    id="basic-url"
                    aria-describedby="basic-addon3"
                  />
                </InputGroup>
                <Form.Label htmlFor="basic-url">Confirm Password</Form.Label>
                <InputGroup className="mb-4">
                  <Form.Control
                    id="basic-url"
                    aria-describedby="basic-addon3"
                  />
                </InputGroup>
              </div>
              <Button href="/menu" variant="success" style={{ width: "100%" }}>
                Confirm
              </Button>
            </Card.Body>
          </Card>
        </div>
      </div>
      <div className={styles.footer}>
        <div style={{ paddingLeft: "100px", paddingTop: "50px" }}>
          <div style={{ paddingBottom: "10px" }}>Facbook: hkm@Webank.com</div>
          <div>line: hkm@Webank.com</div>
        </div>
        <div style={{ paddingRight: "100px", paddingTop: "50px" }}>
          Contact: 09-9999-999
        </div>
      </div>
    </div>
  );
}