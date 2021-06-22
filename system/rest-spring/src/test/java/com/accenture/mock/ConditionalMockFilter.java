package com.accenture.mock;

import org.platformlambda.core.annotations.OptionalService;

import javax.servlet.*;
import javax.servlet.annotation.WebFilter;
import java.io.IOException;

@WebFilter("/none/*")
@OptionalService("no.such.parameter")
public class ConditionalMockFilter implements Filter {

    @Override
    public void doFilter(ServletRequest request, ServletResponse response, FilterChain chain) throws IOException, ServletException {
        // this is a no-op to test if this mock is loaded by the system
        chain.doFilter(request, response);
    }

}
