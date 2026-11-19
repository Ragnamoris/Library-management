/* @refresh reload */
import { render } from "solid-js/web";
import { Router, Route } from "@solidjs/router";

import "./ui/index.css";

import { Root } from "./Root";
import { PublishersPage } from "./pages/PublishersPage";
import { BooksPage } from "./pages/BooksPage";
import { ReadersPage } from "./pages/ReadersPage";
import { LoansPage } from "./pages/LoansPage";

const root = document.getElementById("root");

render(() => (
    <Router root={Root}>
        <Route path="/" component={PublishersPage} />
        <Route path="/publishers" component={PublishersPage} />
        <Route path="/books" component={BooksPage} />
        <Route path="/readers" component={ReadersPage} />
        <Route path="/loans" component={LoansPage} />
    </Router>
), root!);