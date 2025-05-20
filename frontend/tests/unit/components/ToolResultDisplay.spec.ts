import { mount } from '@vue/test-utils';
import ToolResultDisplay from '@/components/ToolResultDisplay.vue';

describe('ToolResultDisplay.vue', () => {
  it('renders image result correctly', () => {
    const mockResult = {
      type: 'image',
      url: 'https://example.com/image.jpg',
      alt: 'Example image',
      caption: 'This is an example image'
    };
    
    const wrapper = mount(ToolResultDisplay, {
      props: {
        result: mockResult
      }
    });
    
    // Check that the image is displayed
    const img = wrapper.find('img');
    expect(img.exists()).toBe(true);
    expect(img.attributes('src')).toBe(mockResult.url);
    expect(img.attributes('alt')).toBe(mockResult.alt);
    
    // Check that the caption is displayed
    const caption = wrapper.find('.image-caption');
    expect(caption.exists()).toBe(true);
    expect(caption.text()).toBe(mockResult.caption);
  });
  
  it('renders table result correctly', () => {
    const mockResult = [
      { id: 1, name: 'John', age: 30 },
      { id: 2, name: 'Jane', age: 25 },
      { id: 3, name: 'Bob', age: 40 }
    ];
    
    const wrapper = mount(ToolResultDisplay, {
      props: {
        result: mockResult
      }
    });
    
    // Check that the table is displayed
    const table = wrapper.find('table');
    expect(table.exists()).toBe(true);
    
    // Check that the headers are displayed
    const headers = wrapper.findAll('th');
    expect(headers.length).toBe(3);
    expect(headers[0].text()).toBe('id');
    expect(headers[1].text()).toBe('name');
    expect(headers[2].text()).toBe('age');
    
    // Check that the rows are displayed
    const rows = wrapper.findAll('tbody tr');
    expect(rows.length).toBe(3);
    
    // Check the first row
    const firstRowCells = rows[0].findAll('td');
    expect(firstRowCells[0].text()).toBe('1');
    expect(firstRowCells[1].text()).toBe('John');
    expect(firstRowCells[2].text()).toBe('30');
  });
  
  it('renders code result correctly', () => {
    const mockResult = {
      type: 'code',
      code: 'console.log("Hello, world!");',
      language: 'javascript'
    };
    
    const wrapper = mount(ToolResultDisplay, {
      props: {
        result: mockResult
      }
    });
    
    // Check that the code is displayed
    const codeResult = wrapper.find('.code-result');
    expect(codeResult.exists()).toBe(true);
    
    // Check that the language is displayed
    const language = wrapper.find('.code-language');
    expect(language.exists()).toBe(true);
    expect(language.text()).toBe(mockResult.language);
    
    // Check that the code is displayed
    const code = wrapper.find('pre code');
    expect(code.exists()).toBe(true);
    expect(code.text()).toBe(mockResult.code);
    
    // Check that the copy button is displayed
    const copyButton = wrapper.find('.copy-button');
    expect(copyButton.exists()).toBe(true);
  });
  
  it('renders HTML result correctly', () => {
    const mockResult = {
      type: 'html',
      html: '<div><h1>Hello, world!</h1><p>This is a test</p></div>'
    };
    
    const wrapper = mount(ToolResultDisplay, {
      props: {
        result: mockResult
      }
    });
    
    // Check that the HTML is displayed
    const htmlResult = wrapper.find('.html-result');
    expect(htmlResult.exists()).toBe(true);
    expect(htmlResult.html()).toContain(mockResult.html);
  });
  
  it('renders link result correctly', () => {
    const mockResult = {
      type: 'link',
      url: 'https://example.com',
      title: 'Example Website',
      description: 'This is an example website'
    };
    
    const wrapper = mount(ToolResultDisplay, {
      props: {
        result: mockResult
      }
    });
    
    // Check that the link is displayed
    const linkResult = wrapper.find('.link-result');
    expect(linkResult.exists()).toBe(true);
    
    // Check that the link has the correct attributes
    const link = wrapper.find('a');
    expect(link.exists()).toBe(true);
    expect(link.attributes('href')).toBe(mockResult.url);
    expect(link.text()).toContain(mockResult.title);
    
    // Check that the description is displayed
    const description = wrapper.find('.link-description');
    expect(description.exists()).toBe(true);
    expect(description.text()).toBe(mockResult.description);
  });
  
  it('renders text result correctly', () => {
    const mockResult = {
      type: 'text',
      text: 'This is a test text result'
    };
    
    const wrapper = mount(ToolResultDisplay, {
      props: {
        result: mockResult
      }
    });
    
    // Check that the text is displayed
    const textResult = wrapper.find('.text-result');
    expect(textResult.exists()).toBe(true);
    
    // Check that the text is displayed
    const text = wrapper.find('.text-result p');
    expect(text.exists()).toBe(true);
    expect(text.text()).toBe(mockResult.text);
  });
  
  it('renders JSON result correctly for unknown result types', () => {
    const mockResult = {
      foo: 'bar',
      baz: 123
    };
    
    const wrapper = mount(ToolResultDisplay, {
      props: {
        result: mockResult
      }
    });
    
    // Check that the JSON is displayed
    const jsonResult = wrapper.find('.json-result');
    expect(jsonResult.exists()).toBe(true);
    
    // Check that the JSON is displayed
    const json = wrapper.find('pre code');
    expect(json.exists()).toBe(true);
    expect(json.text()).toBe(JSON.stringify(mockResult, null, 2));
  });
});
