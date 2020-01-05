package org.platformlambda.automation.util;

import org.platformlambda.automation.models.HeaderInfo;
import org.platformlambda.core.util.Utility;

import javax.servlet.http.HttpServletResponse;
import javax.servlet.http.Part;
import java.util.HashMap;
import java.util.List;
import java.util.Map;

public class SimpleHttpUtility {

    private static final String SET_COOKIE = "set-cookie";
    private static final String COOKIE_SEPARATOR = "|";
    private static final String CONTENT_DISPOSITION = "content-disposition";
    private static final String FILENAME = "filename";

    private static final SimpleHttpUtility instance = new SimpleHttpUtility();

    private SimpleHttpUtility() {
        // singleton
    }

    public static SimpleHttpUtility getInstance() {
        return instance;
    }

    public void setCookies(HttpServletResponse response, String cookies) {
        String header = getHeaderCase(SET_COOKIE);
        if (cookies.contains(COOKIE_SEPARATOR)) {
            List<String> items = Utility.getInstance().split(cookies, COOKIE_SEPARATOR);
            for (String value: items) {
                response.addHeader(header, value);
            }
        } else {
            response.setHeader(header, cookies);
        }
    }

    public String getHeaderCase(String header) {
        StringBuilder sb = new StringBuilder();
        List<String> parts = Utility.getInstance().split(header, "-");
        for (String p: parts) {
            sb.append(p.substring(0, 1).toUpperCase());
            if (p.length() > 1) {
                sb.append(p.substring(1));
            }
            sb.append('-');
        }
        return sb.length() == 0? null : sb.substring(0, sb.length()-1);
    }

    public Map<String, String> filterHeaders(HeaderInfo headerInfo, Map<String, String> headers) {
        Map<String, String> result = new HashMap<>(headers);
        if (headerInfo.keepHeaders != null && !headerInfo.keepHeaders.isEmpty()) {
            // drop all headers except those to be kept
            Map<String, String> toBeKept = new HashMap<>();
            for (String h: headers.keySet()) {
                if (headerInfo.keepHeaders.contains(h)) {
                    toBeKept.put(h, headers.get(h));
                }
            }
            result = toBeKept;
        } else if (headerInfo.dropHeaders != null && !headerInfo.dropHeaders.isEmpty()) {
            // drop the headers according to "drop" list
            Map<String, String> toBeKept = new HashMap<>();
            for (String h: headers.keySet()) {
                if (!headerInfo.dropHeaders.contains(h)) {
                    toBeKept.put(h, headers.get(h));
                }
            }
            result = toBeKept;
        }
        if (headerInfo.additionalHeaders != null && !headerInfo.additionalHeaders.isEmpty()) {
            for (String h: headerInfo.additionalHeaders.keySet()) {
                result.put(h, headerInfo.additionalHeaders.get(h));
            }
        }
        return result;
    }

    public String normalizeUrl(String url, List<String> urlRewrite) {
        if (urlRewrite != null && urlRewrite.size() == 2) {
            if (url.startsWith(urlRewrite.get(0))) {
                return urlRewrite.get(1) + url.substring(urlRewrite.get(0).length());
            }
        }
        return url;
    }

    public String getFileName(final Part part) {
        for (String content : part.getHeader(CONTENT_DISPOSITION).split(";")) {
            if (content.trim().startsWith(FILENAME)) {
                return content.substring(content.indexOf('=') + 1).trim().replace("\"", "");
            }
        }
        return null;
    }

}
