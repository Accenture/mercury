package com.accenture.mock;

import org.platformlambda.core.annotations.OptionalService;

import javax.servlet.annotation.WebServlet;
import javax.servlet.http.HttpServlet;
import javax.servlet.http.HttpServletRequest;
import javax.servlet.http.HttpServletResponse;
import java.io.IOException;

@WebServlet("/mock_servlet_none")
@OptionalService("no.such.parameter")
public class ConditionalHelloServlet extends HttpServlet {

    @Override
    protected void doGet(HttpServletRequest request, HttpServletResponse response) throws IOException {
        response.getWriter().write("hello world from servlet");
    }

}
